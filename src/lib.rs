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
//! use alexa_sdk::{RequestEnvelope,ResponseEnvelope};
//! use alexa_sdk::request::{IntentType, Locale};
//!
//! fn handle_help(_req: &RequestEnvelope) -> Result<ResponseEnvelope,Error> {
//!     Ok(ResponseEnvelope::simple("hello", "to say hello, tell me: say hello to someone"))
//! }
//!
//! fn handle_hello(req: &RequestEnvelope) -> Result<ResponseEnvelope,Error> {
//!     let res = match req.request.locale {
//!         Locale::AustralianEnglish => ResponseEnvelope::simple("hello", "G'day mate"),
//!         Locale::German => ResponseEnvelope::simple("hello", "Hallo Welt"),
//!         Locale::Japanese => ResponseEnvelope::simple("hello", "こんにちは世界"),
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

pub use self::request::RequestEnvelope;
pub use self::response::ResponseEnvelope;

#[macro_export]
macro_rules! declare_api_enum {
    ($rust_name:ident [$convention:literal] { $( $known_value:ident ),* }) => {
        #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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
}