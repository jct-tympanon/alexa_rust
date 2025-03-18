extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use serde::de::Visitor;
use serde::{Deserialize, Serialize};

use crate::declare_api_enum;

use std::collections::HashMap;
use std::convert::From;

/// Request struct corresponding to the [Alexa spec](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html#request-body-parameters)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestEnvelope {
    pub version: String,
    pub session: Option<Session>,
    pub request: Request,
    pub context: Context,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub new: bool,
    pub session_id: String,
    pub attributes: Option<HashMap<String, String>>,
    pub application: Application,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub application_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String,
    pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(rename = "type")]
    pub request_type: RequestType,
    pub request_id: String,
    pub timestamp: String,
    pub locale: Locale,
    pub intent: Option<Intent>,
    pub reason: Option<String>,
    pub dialog_state: Option<String>,
}

/// Partial mapping of Context, 
/// see https://developer.amazon.com/en-US/docs/alexa/custom-skills/request-and-response-json-reference.html#context-object
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")] 
pub struct Context {
    pub system: System,
    pub audio_player: Option<AudioPlayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct System {
    pub api_access_token: Option<String>,
    pub device: Option<Device>,
    pub application: Option<Application>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioPlayer {
    pub token: Option<String>,
    pub offset_in_milliseconds: Option<u64>,
    pub player_activity: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Intent {
    pub name: IntentType,
    pub confirmation_status: Option<String>,
    pub slots: Option<HashMap<String, Slot>>,
}

impl Intent {
    fn get_slot(&self, name: &str) -> Option<&Slot> {
        self.slots.as_ref()?.get(name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub confirmation_status: Option<String>,
    pub resolutions: Option<Resolution>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resolution {
    pub resolutions_per_authority: Vec<ResolutionsPerAuthority>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResolutionsPerAuthority {
    pub authority: String,
    pub status: Status,
    pub values: Vec<ValueWrapper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueWrapper {
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub name: String,
    pub id: String,
}

declare_api_enum! {
    RequestType["PascalCase"] {
        LaunchRequest,
        IntentRequest,
        SessionEndedRequest,
        CanFulfillIntentRequest
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum IntentType {
    #[serde(rename = "AMAZON.HelpIntent")]
    Help,
    #[serde(rename = "AMAZON.CancelIntent")]
    Cancel,
    #[serde(rename = "AMAZON.FallbackIntent")]
    Fallback,
    #[serde(rename = "AMAZON.LoopOffIntent")]
    LoopOff,
    #[serde(rename = "AMAZON.LoopOnIntent")]
    LoopOn,
    #[serde(rename = "AMAZON.NavigateHomeIntent")]
    NavigateHome,
    #[serde(rename = "AMAZON.NextIntent")]
    Next,
    #[serde(rename = "AMAZON.NoIntent")]
    No,
    #[serde(rename = "AMAZON.PauseIntent")]
    Pause,
    #[serde(rename = "AMAZON.PreviousIntent")]
    Previous,
    #[serde(rename = "AMAZON.RepeatIntent")]
    Repeat,
    #[serde(rename = "AMAZON.ResumeIntent")]
    Resume,
    #[serde(rename = "AMAZON.SelectIntent")]
    Select,
    #[serde(rename = "AMAZON.ShuffleOffIntent")]
    ShuffleOff,
    #[serde(rename = "AMAZON.ShuffleOnIntent")]
    ShuffleOn,
    #[serde(rename = "AMAZON.StartOverIntent")]
    StartOver,
    #[serde(rename = "AMAZON.StopIntent")]
    Stop,
    #[serde(rename = "AMAZON.YesIntent")]
    Yes,
    #[serde(untagged)]
    Other(String),
}

/// Alexa standard locales
#[derive(Debug, PartialEq, Clone)]
pub enum Locale {
    Italian,
    German,
    AustralianEnglish,
    CanadianEnglish,
    BritishEnglish,
    IndianEnglish,
    AmericanEnglish,
    Japanese,
    Spanish,
    MexicanSpanish,
    AmericanSpanish,
    Hindi,
    French,
    CanadianFrench,
    BrazilianPortuguese,
    Other(String),
}

impl Locale {
    /// returns true for all English speaking locals
    pub fn is_english(&self) -> bool {
        match *self {
            Locale::AmericanEnglish => true,
            Locale::AustralianEnglish => true,
            Locale::CanadianEnglish => true,
            Locale::BritishEnglish => true,
            Locale::IndianEnglish => true,
            _ => false,
        }
    }
    pub fn is_french(&self) -> bool {
        match *self {
            Locale::French => true,
            Locale::CanadianFrench => true,
            _ => false,
        }
    }
    pub fn is_spanish(&self) -> bool {
        match *self {
            Locale::Spanish => true,
            Locale::AmericanSpanish => true,
            Locale::MexicanSpanish => true,
            _ => false,
        }
    }

    pub fn as_str(&self) -> &str {
        match *self {
            Locale::Italian => "it-IT",
            Locale::German => "de-DE",
            Locale::AustralianEnglish => "en-AU",
            Locale::CanadianEnglish => "en-CA",
            Locale::BritishEnglish => "en-GB",
            Locale::IndianEnglish => "en-IN",
            Locale::AmericanEnglish => "en-US",
            Locale::Japanese => "ja-JP",
            Locale::Hindi => "hi-HI",
            Locale::Spanish => "es-ES",
            Locale::MexicanSpanish => "es-MX",
            Locale::AmericanSpanish => "es-US",
            Locale::French => "fr-FR",
            Locale::CanadianFrench => "fr-CA",
            Locale::BrazilianPortuguese => "pt-BR",
            Locale::Other(ref s) => s,
        }
    }
}
impl<S> From<S> for Locale where S: AsRef<str> {
    fn from(s: S) -> Locale {
        match s.as_ref() {
            "it-IT" => Locale::Italian,
            "de-DE" => Locale::German,
            "en-AU" => Locale::AustralianEnglish,
            "en-CA" => Locale::CanadianEnglish,
            "en-GB" => Locale::BritishEnglish,
            "en-IN" => Locale::IndianEnglish,
            "en-US" => Locale::AmericanEnglish,
            "ja-JP" => Locale::Japanese,
            "hi-HI" => Locale::Hindi,
            "es-ES" => Locale::Spanish,
            "es-MX" => Locale::MexicanSpanish,
            "es-US" => Locale::AmericanSpanish,
            "fr-FR" => Locale::French,
            "fr-CA" => Locale::CanadianFrench,
            "pt-BR" => Locale::BrazilianPortuguese,
            s @ _ => Locale::Other(s.to_string()),
        }
    }
}
impl Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {

        struct LocaleVisitor;
        impl<'de> Visitor<'de> for LocaleVisitor {
            type Value = Locale;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a two-part locale string, {{language}}-{{country}}")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: serde::de::Error, {
                Ok(v.into())
            }
        }
        
        deserializer.deserialize_str(LocaleVisitor)
    }
}

impl RequestEnvelope {
    pub fn intent_type(&self) -> Option<&IntentType> {
        self.request.intent.as_ref().map(|i| &i.name)
    }

    /// retrieves the string value of named slot from the request, if it exists
    pub fn slot_value(&self, slot: &str) -> Option<&String> {
        self.request
            .intent.as_ref()?
            .get_slot(slot)?
            .value.as_ref()
    }

    /// retrieves the attribute value with the given key, if it exists
    pub fn attribute_value(&self, key: &str) -> Option<&String> {
        self.session.as_ref()?.attributes.as_ref()?.get(key)
    }

    /// returns whether or not this is a new request
    pub fn is_new(&self) -> bool {
        match &self.session {
            Some(s) => s.new,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_version() {
        let req: RequestEnvelope = serde_json::from_value(default_req()).unwrap();
        assert_eq!(req.version, "1.0");
    }

    #[test]
    fn test_locale() {
        let req: RequestEnvelope = serde_json::from_value(default_req()).unwrap();
        assert_eq!(req.request.locale, Locale::AmericanEnglish);
    }

    #[test]
    fn test_is_english() {
        let req: RequestEnvelope = serde_json::from_value(default_req()).unwrap();
        assert!(req.request.locale.is_english());
    }

    #[test]
    fn test_is_spanish() {
        let req: RequestEnvelope = serde_json::from_value(default_spanish_req()).unwrap();
        assert!(req.request.locale.is_spanish());
    }

    #[test]
    fn test_is_french() {
        let req: RequestEnvelope = serde_json::from_value(default_french_req()).unwrap();
        assert!(req.request.locale.is_french());
    }

    #[test]
    fn test_intent() {
        let req: RequestEnvelope = serde_json::from_value(default_req()).unwrap();
        assert_eq!(req.intent_type(), Some(&IntentType::Other(String::from("hello"))));
    }

    #[test]
    fn test_slot() {
        let req: RequestEnvelope = serde_json::from_value(req_with_slots()).unwrap();
        assert_eq!(req.slot_value("name"), Some(&String::from("bob")));
    }

    #[test]
    fn test_attribute() {
        let req: RequestEnvelope = serde_json::from_value(default_req()).unwrap();
        assert!(req.session.is_some());
        assert!(req.session.unwrap().attributes.is_some());
    }

    #[test]
    fn test_attribute_val() {
        let req: RequestEnvelope = serde_json::from_value(default_req()).unwrap();
        assert_eq!(
            req.attribute_value("lastSpeech"),
            Some(&String::from(
                "Jupiter has the shortest day of all the planets"
            ))
        );
    }

    #[test]
    fn deserialize_playback_intent() {
        let req: RequestEnvelope = serde_json::from_value(with_playback_intent()).unwrap();
        assert_eq!(
            req.slot_value("object.name"),
            Some(&String::from("in rainbows"))
        );
    }

    fn default_spanish_req() -> serde_json::Value {
        json!({
            "version": "1.0",
            "session": {
                "new": true,
                "sessionId": "amzn1.echo-api.session.abc123",
                "application": {
                    "applicationId": "amzn1.ask.skill.myappid"
                },
                "attributes": {
                    "lastSpeech": "Jupiter has the shortest day of all the planets"
                },
                "user": {
                    "userId": "amzn1.ask.account.theuserid"
                }
            },
            "context": {
                "System": {
                    "application": {
                        "applicationId": "amzn1.ask.skill.myappid"
                    },
                    "user": {
                        "userId": "amzn1.ask.account.theuserid"
                    },
                    "device": {
                        "deviceId": "amzn1.ask.device.superfakedevice",
                        "supportedInterfaces": {}
                    },
                    "apiEndpoint": "https://api.amazonalexa.com",
                    "apiAccessToken": "53kr14t.k3y.d4t4-otherstuff"
                },
                "Viewport": {
                    "experiences": [
                        {
                            "arcMinuteWidth": 246,
                            "arcMinuteHeight": 144,
                            "canRotate": false,
                            "canResize": false
                        }
                    ],
                    "shape": "RECTANGLE",
                    "pixelWidth": 1024,
                    "pixelHeight": 600,
                    "dpi": 160,
                    "currentPixelWidth": 1024,
                    "currentPixelHeight": 600,
                    "touch": [
                        "SINGLE"
                    ]
                }
            },
            "request": {
                "type": "IntentRequest",
                "requestId": "amzn1.echo-api.request.b8b49fde-4370-423f-bbb0-dc7305b788a0",
                "timestamp": "2018-12-03T00:33:58Z",
                "locale": "es-MX",
                "intent": {
                    "name": "hello",
                    "confirmationStatus": "NONE"
                }
            }
        })
    }

    fn default_french_req() -> serde_json::Value {
        json!({
            "version": "1.0",
            "session": {
                "new": true,
                "sessionId": "amzn1.echo-api.session.abc123",
                "application": {
                    "applicationId": "amzn1.ask.skill.myappid"
                },
                "attributes": {
                    "lastSpeech": "Jupiter has the shortest day of all the planets"
                },
                "user": {
                    "userId": "amzn1.ask.account.theuserid"
                }
            },
            "context": {
                "System": {
                    "application": {
                        "applicationId": "amzn1.ask.skill.myappid"
                    },
                    "user": {
                        "userId": "amzn1.ask.account.theuserid"
                    },
                    "device": {
                        "deviceId": "amzn1.ask.device.superfakedevice",
                        "supportedInterfaces": {}
                    },
                    "apiEndpoint": "https://api.amazonalexa.com",
                    "apiAccessToken": "53kr14t.k3y.d4t4-otherstuff"
                },
                "Viewport": {
                    "experiences": [
                        {
                            "arcMinuteWidth": 246,
                            "arcMinuteHeight": 144,
                            "canRotate": false,
                            "canResize": false
                        }
                    ],
                    "shape": "RECTANGLE",
                    "pixelWidth": 1024,
                    "pixelHeight": 600,
                    "dpi": 160,
                    "currentPixelWidth": 1024,
                    "currentPixelHeight": 600,
                    "touch": [
                        "SINGLE"
                    ]
                }
            },
            "request": {
                "type": "IntentRequest",
                "requestId": "amzn1.echo-api.request.b8b49fde-4370-423f-bbb0-dc7305b788a0",
                "timestamp": "2018-12-03T00:33:58Z",
                "locale": "fr-CA",
                "intent": {
                    "name": "hello",
                    "confirmationStatus": "NONE"
                }
            }
        })
    }

    fn default_req() -> serde_json::Value {
        json!({
            "version": "1.0",
            "session": {
                "new": true,
                "sessionId": "amzn1.echo-api.session.abc123",
                "application": {
                    "applicationId": "amzn1.ask.skill.myappid"
                },
                "attributes": {
                    "lastSpeech": "Jupiter has the shortest day of all the planets"
                },
                "user": {
                    "userId": "amzn1.ask.account.theuserid"
                }
            },
            "context": {
                "System": {
                    "application": {
                        "applicationId": "amzn1.ask.skill.myappid"
                    },
                    "user": {
                        "userId": "amzn1.ask.account.theuserid"
                    },
                    "device": {
                        "deviceId": "amzn1.ask.device.superfakedevice",
                        "supportedInterfaces": {}
                    },
                    "apiEndpoint": "https://api.amazonalexa.com",
                    "apiAccessToken": "53kr14t.k3y.d4t4-otherstuff"
                },
                "Viewport": {
                    "experiences": [
                        {
                            "arcMinuteWidth": 246,
                            "arcMinuteHeight": 144,
                            "canRotate": false,
                            "canResize": false
                        }
                    ],
                    "shape": "RECTANGLE",
                    "pixelWidth": 1024,
                    "pixelHeight": 600,
                    "dpi": 160,
                    "currentPixelWidth": 1024,
                    "currentPixelHeight": 600,
                    "touch": [
                        "SINGLE"
                    ]
                }
            },
            "request": {
                "type": "IntentRequest",
                "requestId": "amzn1.echo-api.request.b8b49fde-4370-423f-bbb0-dc7305b788a0",
                "timestamp": "2018-12-03T00:33:58Z",
                "locale": "en-US",
                "intent": {
                    "name": "hello",
                    "confirmationStatus": "NONE"
                }
            }
        })
    }

    fn req_with_slots() -> serde_json::Value {
        json!({
            "version": "1.0",
            "session": {
                "new": true,
                "sessionId": "amzn1.echo-api.session.blahblahblah",
                "application": {
                    "applicationId": "amzn1.ask.skill.testappliction"
                },
                "user": {
                    "userId": "amzn1.ask.account.longstringuseridentifier"
                }
            },
            "context": {
                "Display": {},
                "System": {
                    "application": {
                        "applicationId": "amzn1.ask.skill.tehappz"
                    },
                    "user": {
                        "userId": "amzn1.ask.account.longstringuseridentifier"
                    },
                    "device": {
                        "deviceId": "amzn1.ask.device.testdevice",
                        "supportedInterfaces": {
                            "Display": {
                                "templateVersion": "1.0",
                                "markupVersion": "1.0"
                            }
                        }
                    },
                    "apiEndpoint": "https://api.amazonalexa.com",
                    "apiAccessToken": "teh.token.with-long-string-more-more-more-more"
                },
                "Viewport": {
                    "experiences": [
                        {
                            "arcMinuteWidth": 246,
                            "arcMinuteHeight": 144,
                            "canRotate": false,
                            "canResize": false
                        }
                    ],
                    "shape": "RECTANGLE",
                    "pixelWidth": 1024,
                    "pixelHeight": 600,
                    "dpi": 160,
                    "currentPixelWidth": 1024,
                    "currentPixelHeight": 600,
                    "touch": [
                        "SINGLE"
                    ]
                }
            },
            "request": {
                "type": "IntentRequest",
                "requestId": "amzn1.echo-api.request.id",
                "timestamp": "2018-12-08T05:37:32Z",
                "locale": "en-US",
                "intent": {
                    "name": "hello",
                    "confirmationStatus": "NONE",
                    "slots": {
                        "name": {
                            "name": "name",
                            "value": "bob",
                            "confirmationStatus": "NONE",
                            "source": "USER"
                        }
                    }
                }
            }
        })
    }

    /// a live example with redacted identifiers.
    fn with_playback_intent() -> serde_json::Value {
        json!({
            "version": "1.0",
            "session": {
                "new": true,
                "sessionId": "amzn1.echo-api.session.SESSION",
                "application": {
                    "applicationId": "amzn1.ask.skill.APP"
                },
                "attributes": {},
                "user": {
                    "userId": "amzn1.ask.account.USER"
                }
            },
            "context": {
                "Viewports": [
                    {
                        "type": "APL",
                        "id": "medHub",
                        "shape": "RECTANGLE",
                        "dpi": 160,
                        "presentationType": "OVERLAY",
                        "canRotate": false,
                        "configuration": {
                            "current": {
                                "mode": "HUB",
                                "video": {
                                    "codecs": [
                                        "H_264_42",
                                        "H_264_41"
                                    ]
                                },
                                "size": {
                                    "type": "DISCRETE",
                                    "pixelWidth": 1280,
                                    "pixelHeight": 800
                                }
                            }
                        }
                    }
                ],
                "AudioPlayer": {
                    "playerActivity": "IDLE"
                },
                "Viewport": {
                    "experiences": [
                        {
                            "arcMinuteWidth": 221,
                            "arcMinuteHeight": 162,
                            "canRotate": false,
                            "canResize": false
                        }
                    ],
                    "mode": "HUB",
                    "shape": "RECTANGLE",
                    "pixelWidth": 1280,
                    "pixelHeight": 800,
                    "dpi": 160,
                    "currentPixelWidth": 1280,
                    "currentPixelHeight": 800,
                    "touch": [
                        "SINGLE"
                    ],
                    "keyboard": [
                        "DIRECTION"
                    ],
                    "video": {
                        "codecs": [
                            "H_264_42",
                            "H_264_41"
                        ]
                    }
                },
                "Extensions": {
                    "available": {
                        "aplext:backstack:10": {}
                    }
                },
                "System": {
                    "application": {
                        "applicationId": "amzn1.ask.skill.APP"
                    },
                    "user": {
                        "userId": "amzn1.ask.account.USER"
                    },
                    "device": {
                        "deviceId": "amzn1.ask.device.DEVICE",
                        "supportedInterfaces": {
                            "AudioPlayer": {}
                        }
                    },
                    "apiEndpoint": "https://api.amazonalexa.com",
                    "apiAccessToken": "SECRET"
                }
            },
            "request": {
                "type": "IntentRequest",
                "requestId": "amzn1.echo-api.request.REQUEST",
                "locale": "en-US",
                "timestamp": "2025-03-17T23:27:29Z",
                "intent": {
                    "name": "AMAZON.PlaybackAction<object@MusicCreativeWork>",
                    "confirmationStatus": "NONE",
                    "slots": {
                        "object.era": {
                            "name": "object.era",
                            "confirmationStatus": "NONE"
                        },
                        "object.name": {
                            "name": "object.name",
                            "value": "in rainbows",
                            "confirmationStatus": "NONE",
                            "source": "USER",
                            "slotValue": {
                                "type": "Simple",
                                "value": "in rainbows"
                            }
                        },
                        "object.sort": {
                            "name": "object.sort",
                            "confirmationStatus": "NONE"
                        },
                        "object.byArtist.name": {
                            "name": "object.byArtist.name",
                            "confirmationStatus": "NONE"
                        },
                        "object.select": {
                            "name": "object.select",
                            "confirmationStatus": "NONE"
                        },
                        "object.type": {
                            "name": "object.type",
                            "confirmationStatus": "NONE"
                        },
                        "object.genre": {
                            "name": "object.genre",
                            "confirmationStatus": "NONE"
                        },
                        "object.owner.name": {
                            "name": "object.owner.name",
                            "confirmationStatus": "NONE"
                        },
                        "object.composer.name": {
                            "name": "object.composer.name",
                            "confirmationStatus": "NONE"
                        },
                        "object.contentSource": {
                            "name": "object.contentSource",
                            "confirmationStatus": "NONE"
                        }
                    }
                }
            }
        })
    }
}
