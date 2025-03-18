//! # Alexa SDK
//! 
//! `alexa_sdk` implements stucts corresponding to the [Alexa JSON specification](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html)
//! along with helper functions for common uses of the  `RequestEnvelope` and `ResponseEnvelope` objects.
//! 
//! ## Usage
//! 
//! Simplest possible Alexa "Hello, World" skill:
//!
//! ```rust
//! use lambda_runtime::{service_fn, Error, LambdaEvent};
//! use alexa_sdk::{RequestEnvelope,ResponseEnvelope};
//!
//! async fn my_handler(event: LambdaEvent<RequestEnvelope>) -> Result<ResponseEnvelope, Error> {
//!     let (_req, _ctx) = event.into_parts();
//!     Ok(ResponseEnvelope::simple("hello", "hello world"))
//! }
//!
//! # #[cfg(feature = "doctest")]
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     lambda_runtime::run(service_fn(my_handler)).await?;
//!     Ok(())
//! }
//! 
//! # fn main() {}
//! ```
//!
//! A more complete skill, handling multiple locales and a slot:
//!
//! ```rust
//! use lambda_runtime::{service_fn, Error, LambdaEvent};
//! use alexa_sdk::{RequestEnvelope, ResponseEnvelope};
//! use alexa_sdk::request::{IntentType, Locale, Language, Region};
//!
//! fn handle_help(_req: &RequestEnvelope) -> Result<ResponseEnvelope,Error> {
//!     Ok(ResponseEnvelope::simple("hello", "to say hello, tell me: say hello to someone"))
//! }
//!
//! fn handle_hello(req: &RequestEnvelope) -> Result<ResponseEnvelope,Error> {
//!     let res = match req.request.locale.parts() {
//!         (&Language::English, Some(&Region::Australia)) => ResponseEnvelope::simple("hello", "G'day mate"),
//!         (&Language::German, _) => ResponseEnvelope::simple("hello", "Hallo Welt"),
//!         (&Language::Japanese, _) => ResponseEnvelope::simple("hello", "こんにちは世界"),
//!         _ => if let Some(ref s) = req.slot_value("name") {
//!             ResponseEnvelope::simple("hello", (String::from("hello ") + s).as_str())
//!         } else {
//!             ResponseEnvelope::simple("hello", "hello world")
//!         },
//!     };
//!     Ok(res)
//! }
//!
//! fn handle_cancel(_req: &RequestEnvelope) -> Result<ResponseEnvelope,Error> {
//!     Ok(ResponseEnvelope::end())
//! }
//!
//! async fn my_handler(event: LambdaEvent<RequestEnvelope>) -> Result<ResponseEnvelope,Error> {
//!     let (env, _ctx) = event.into_parts();
//!     match env.intent_type() {
//!         Some(IntentType::Help) => handle_help(&env),
//!         Some(IntentType::Other(_)) => handle_hello(&env),
//!         _ => handle_cancel (&env)
//!     }
//! }
//! 
//! # #[cfg(feature = "doctest")]
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     lambda_runtime::run(service_fn(my_handler)).await?;
//!     Ok(())
//! }
//! 
//! # fn main() {}
//! ```

pub mod request;
pub mod response;

#[cfg(feature = "audioplayer")]
pub mod audioplayer;

#[cfg(feature = "display")]
pub mod display;

pub use self::request::RequestEnvelope;
pub use self::response::ResponseEnvelope;

/// Declares a public rust enum type corresponding to an enumerated string type in the Alexa SDK spec.
/// See for example [`request::IntentType`], [`response::PlayBehavior`], and so on. API enum types have 
/// the following properties:
/// 
/// - All have an extra variant called "Other" which contains any unrecognized value.
/// - All can serialize to and from any string literal without errors.
/// 
/// There are three supported ways to declare an enum type with this macro:
/// ## 1. Simple declarations
/// 
/// The rust enum variants are all written in PascalCase, and the string literals match exactly:
/// ```
/// use alexa_sdk::declare_api_enum;
/// 
/// declare_api_enum! {
///     RequestType {
///         LaunchRequest,
///         IntentRequest
///     }
/// };
/// 
/// assert_eq!("\"LaunchRequest\"", serde_json::to_string(&RequestType::LaunchRequest).unwrap());
/// ```
/// 
/// ## 2. Declarations with a different case convention
/// 
/// The Alexa SDK string constants are something other than PascalCase, but all of them obey the same pattern.
/// Any convention supported by [serde rename_all](https://serde.rs/container-attrs.html#rename_all) is allowed:
/// ```
/// use alexa_sdk::declare_api_enum;
/// 
/// declare_api_enum! {
///     PlayBehavior => "SCREAMING_SNAKE_CASE" {
///         Enqueue,
///         ReplaceAll
///     }
/// };
/// 
/// assert_eq!("\"REPLACE_ALL\"", serde_json::to_string(&PlayBehavior::ReplaceAll).unwrap());
/// ```
/// ## 3. Explicit mapping
/// 
/// We manually assign each enum variant to a string literal, using rust match arm syntax.
/// ```
/// use alexa_sdk::declare_api_enum;
/// 
/// declare_api_enum! {
///     Language {
///         English => "en",
///         French => "fr"
///     }
/// };
/// 
/// assert_eq!("\"en\"", serde_json::to_string(&Language::English).unwrap());
/// ```
/// 
#[macro_export]
macro_rules! declare_api_enum {
    ($rust_name:ident { $( $known_value:ident ),* }) => {
        declare_api_enum!{ $rust_name => "PascalCase" { $($known_value),* } }
    };

    ($rust_name:ident => $convention:literal { $( $known_value:ident ),* }) => {
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, Debug, PartialEq)]
        #[serde(rename_all = $convention)]
        pub enum $rust_name {
            $(
                $known_value
            ),*,
            /// Any unrecognized value for this enum
            #[serde(untagged)]
            Other(String)
        }
    };

    ($rust_name:ident { $( $known_value:ident => $text:literal ),* }) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $rust_name {
            $(
                $known_value
            ),*,
            /// Any unrecognized value for this enum
            Other(String)
        }
        impl $rust_name {
            pub fn as_str(&self) -> &str {
                match *self {
                    $(
                        Self::$known_value => $text,
                    )*
                    Self::Other(ref s) => s,
                }
            }
        }
        impl<S: AsRef<str>> From<S> for $rust_name {
            fn from(value: S) -> Self {
                match value.as_ref() {
                    $(
                        $text => Self::$known_value,
                    )*
                    s @ _ => Self::Other(s.to_string()),
                }
            }
        }
        impl ::serde::Serialize for $rust_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer {
                    serializer.serialize_str(self.as_str())        
            }
        }
        impl<'de> ::serde::Deserialize<'de> for $rust_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de> {
                
                struct VisitEnum;
                impl<'de> ::serde::de::Visitor<'de> for VisitEnum {
                    type Value = $rust_name;
                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(formatter, concat!("a string literal for ", stringify!($rust_name)))
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where E: serde::de::Error, {
                        Ok(v.into())
                    }
                }
                deserializer.deserialize_str(VisitEnum)
            }
        }
    };
}

#[cfg(test)]
mod tests {

    use crate::response::{CardType, PlayBehavior};

    #[test]
    fn enum_serde_known() {
        assert_eq!("\"LinkAccount\"", serde_json::to_string(&CardType::LinkAccount).unwrap());
        assert_eq!(CardType::LinkAccount, serde_json::from_str("\"LinkAccount\"").unwrap());
    }

    #[test]
    fn enum_serde_unknown() {
        assert_eq!("\"FooBar\"", serde_json::to_string(&CardType::Other("FooBar".into())).unwrap());
        assert_eq!(CardType::Other("FooBar".into()), serde_json::from_str("\"FooBar\"").unwrap());
    }

    #[test]
    fn enum_serde_renames() {
        assert_eq!("\"REPLACE_ALL\"", serde_json::to_string(&PlayBehavior::ReplaceAll).unwrap());
        assert_eq!(PlayBehavior::ReplaceAll, serde_json::from_str("\"REPLACE_ALL\"").unwrap());

        assert_eq!("\"FOO_BAR\"", serde_json::to_string(&PlayBehavior::Other("FOO_BAR".into())).unwrap());
        assert_eq!(PlayBehavior::Other("FOO_BAR".into()), serde_json::from_str("\"FOO_BAR\"").unwrap());
    }

}