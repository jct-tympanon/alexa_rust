//! Alexa SDK AudioPlayer interface datatypes, from [the specification](https://developer.amazon.com/en-US/docs/alexa/custom-skills/audioplayer-interface-reference.html).

use serde::Deserialize;
use serde::Serialize;

use crate::response::{Directive, PlayBehavior};

use super::display::Image;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayDirective {
    pub audio_item: AudioItem,
    pub play_behavior: PlayBehavior,
}
impl From<PlayDirective> for Directive {
    fn from(value: PlayDirective) -> Self {
        Directive::Play(value)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioItem {
    pub stream: Stream,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AudioItemMetadata>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stream {
    pub url: String,
    pub token: String,
    pub offset_in_milliseconds: i64, // should be non-zero positive, but Alexa has been observed to send -1 for this value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_previous_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_data: Option<CaptionData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CaptionData {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    pub content: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioItemMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub art: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<Image>,
}
