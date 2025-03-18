use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::declare_api_enum;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Version {
    #[serde(rename = "1.0")]
    V1_0,
    #[serde(untagged)]
    Other(String)
}

impl ResponseEnvelope {
    /// Constructs a new response with only required elements
    pub fn new(should_end: bool) -> Self {
        Self {
            version: Version::V1_0,
            session_attributes: None,
            response: Response {
                output_speech: None,
                card: None,
                reprompt: None,
                should_end_session: should_end,
                directives: None
            },
        }
    }

    /// Constructs a basic plain response with a simple card
    pub fn new_simple(title: &str, text: &str) -> Self {
        Self::simple(title, text)
    }

    /// Constructs a basic plain response with a simple card
    pub fn simple(title: &str, text: &str) -> Self {
        Self::new(true)
            .card(Card::simple(title, text))
            .speech(Speech::plain(text))
    }

    /// Constructs an empty response ending the session
    pub fn end() -> Self {
        Self::new(true)
    }

    /// adds a speach element to the response
    pub fn speech(mut self, speech: Speech) -> Self {
        self.response.output_speech = Some(speech);
        self
    }

    /// adds a card to the response
    pub fn card(mut self, card: Card) -> Self {
        self.response.card = Some(card);
        self
    }

    /// adds an attribute key/value pair to the response
    /// attributes can be read on the next request for basic state
    /// persistance
    pub fn add_attribute(&mut self, key: &str, val: &str) {
        if let Some(ref mut h) = self.session_attributes {
            let _ = h.insert(String::from(key), String::from(val));
        } else {
            let mut h = HashMap::new();
            h.insert(String::from(key), String::from(val));
            self.session_attributes = Some(h)
        }
    }
}

/// Response struct implementing the [Alexa JSON spec](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html#response-parameters)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseEnvelope {
    pub version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<HashMap<String, String>>,
    pub response: Response,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_speech: Option<Speech>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<Reprompt>,
    pub should_end_session: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directives: Option<Vec<Directive>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Directive {
    #[cfg(feature = "audioplayer")]
    #[serde(rename = "AudioPlayer.Play")]
    Play(crate::audioplayer::PlayDirective),

    #[cfg(feature = "audioplayer")]
    #[serde(rename = "AudioPlayer.Stop")]
    Stop,

    #[serde(untagged)]
    Other(serde_json::Value)
}

declare_api_enum! {
    SpeechType["PascalCase"] {
        PlainText,
        SSML
    }
}
declare_api_enum! {
    PlayBehavior["SCREAMING_SNAKE_CASE"] {
        Enqueue,
        ReplaceAll,
        ReplaceEnqueued
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Speech {
    #[serde(rename = "type")]
    pub speech_type: SpeechType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_behavior: Option<PlayBehavior>,
}

impl Speech {
    /// Constructs a plain text output speech
    pub fn plain(s: &str) -> Speech {
        Speech {
            speech_type: SpeechType::PlainText,
            text: Some(String::from(s)),
            ssml: None,
            play_behavior: None,
        }
    }

    /// Constructs an SSML output speech (with supplied SSML)
    pub fn ssml(s: &str) -> Speech {
        Speech {
            speech_type: SpeechType::SSML,
            ssml: Some(String::from(s)),
            text: None,
            play_behavior: None,
        }
    }

    /// Adds play behavior to a speech object
    pub fn play_behavior(&mut self, behavior: PlayBehavior) {
        self.play_behavior = Some(behavior);
    }
}

declare_api_enum! {
    CardType["PascalCase"] {
        Simple,
        Standard,
        LinkAccount,
        AskForPermissionsConsent
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(rename = "type")]
    pub card_type: CardType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

impl Card {
    /// Constructs a simple card for an Alexa repsonse object
    pub fn simple(title: &str, text: &str) -> Card {
        Card {
            card_type: CardType::Simple,
            title: Some(String::from(title)),
            content: Some(String::from(text)),
            text: None,
            image: None,
            permissions: None,
        }
    }

    /// Constructs a standard card for an Alexa response object
    pub fn standard(title: &str, text: &str, image: Image) -> Card {
        Card {
            card_type: CardType::Standard,
            title: Some(String::from(title)),
            content: None,
            text: Some(String::from(text)),
            image: Some(image),
            permissions: None,
        }
    }

    /// Constructs a link account card for the Alexa response object
    pub fn link_account() -> Card {
        Card {
            card_type: CardType::LinkAccount,
            title: None,
            content: None,
            text: None,
            image: None,
            permissions: None,
        }
    }

    /// Constructs a permissions request card with the requested permissions
    pub fn ask_for_permission(permissions: Vec<String>) -> Card {
        Card {
            card_type: CardType::AskForPermissionsConsent,
            title: None,
            content: None,
            text: None,
            image: None,
            permissions: Some(permissions),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reprompt {
    pub output_speech: Speech,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
}

impl Image {
    pub fn new() -> Image {
        Image::default()
    }

    pub fn small_image_url(mut self, url: String) -> Self {
        self.small_image_url = Some(url);
        self
    }

    pub fn large_image_url(mut self, url: String) -> Self {
        self.large_image_url = Some(url);
        self
    }
}

impl Default for Image {
    fn default() -> Self {
        Image {
            small_image_url: None,
            large_image_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let r: ResponseEnvelope = ResponseEnvelope::simple("hello, world", "hello, dude");
        assert_eq!(r.version, Version::V1_0);
    }

    #[test]
    fn test_builder() {
        let mut res = ResponseEnvelope::new(false)
            .card(Card::standard(
                "foo",
                "bar",
                Image {
                    small_image_url: Some(String::from("baaz.png")),
                    large_image_url: Some(String::from("baazLarge.png")),
                },
            ))
            .speech(Speech::plain("hello"));
        res.add_attribute("attr", "value");
        let t = res.response.card.as_ref().unwrap().title.as_ref().unwrap();
        assert_eq!(t, "foo");
        let txt = res.response.card.as_ref().unwrap().text.as_ref().unwrap();
        assert_eq!(txt, "bar");
        let attr = res
            .session_attributes
            .as_ref()
            .unwrap()
            .get("attr")
            .unwrap();
        assert_eq!(attr, "value");
    }

    #[test]
    fn test_builder_with_image_builder() {
        let mut res = ResponseEnvelope::new(false)
            .card(Card::standard(
                "foo",
                "bar",
                Image::new()
                    .small_image_url(String::from("baaz.png"))
                    .large_image_url(String::from("baazLarge.png")),
            ))
            .speech(Speech::plain("hello"));
        res.add_attribute("attr", "value");
        let t = res.response.card.as_ref().unwrap().title.as_ref().unwrap();
        assert_eq!(t, "foo");
        let txt = res.response.card.as_ref().unwrap().text.as_ref().unwrap();
        assert_eq!(txt, "bar");
        let small_img = res
            .response
            .card
            .as_ref()
            .unwrap()
            .image
            .as_ref()
            .unwrap()
            .small_image_url
            .as_ref()
            .unwrap();
        let large_img = res
            .response
            .card
            .as_ref()
            .unwrap()
            .image
            .as_ref()
            .unwrap()
            .large_image_url
            .as_ref()
            .unwrap();

        assert_eq!(small_img, "baaz.png");
        assert_eq!(large_img, "baazLarge.png");

        let attr = res
            .session_attributes
            .as_ref()
            .unwrap()
            .get("attr")
            .unwrap();
        assert_eq!(attr, "value");
    }

    #[test]
    fn test_title() {
        let t = "hello, world";
        let r = ResponseEnvelope::simple(t, "hello, dude");

        assert_eq!(r.response.card.unwrap().title.unwrap(), t);
    }

    #[test]
    fn test_text() {
        let t = "hello, dude";
        let r = ResponseEnvelope::simple("hello,world", t);

        assert_eq!(r.response.card.unwrap().content.unwrap(), t);
    }

    #[test]
    fn test_should_end() {
        let r = ResponseEnvelope::simple("foo", "bar");
        assert_eq!(r.response.should_end_session, true);
    }
}
