//! # Alexa SDK
//! 
//! `alexa_sdk` implements stucts corresponding to the [Alexa JSON specification](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html)
//! along with helper functions for common uses of the  `Request` and `Response` objects.
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
//!     let res = match req.locale() {
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
//!     let (req, _ctx) = event.into_parts();
//!     match req.intent() {
//!         IntentType::Help => handle_help(&req),
//!         IntentType::User(_) => handle_hello(&req),
//!         _ => handle_cancel (&req)
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
