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
//! use alexa_sdk::{Request,Response};
//!
//! async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
//!     let (_req, _ctx) = event.into_parts();
//!     Ok(Response::simple("hello", "hello world"))
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
//! use alexa_sdk::{Request,Response};
//! use alexa_sdk::request::{IntentType, Locale};
//!
//! fn handle_help(_req: &Request) -> Result<Response,Error> {
//!     Ok(Response::simple("hello", "to say hello, tell me: say hello to someone"))
//! }
//!
//! fn handle_hello(req: &Request) -> Result<Response,Error> {
//!     let res = match req.locale() {
//!         Locale::AustralianEnglish => Response::simple("hello", "G'day mate"),
//!         Locale::German => Response::simple("hello", "Hallo Welt"),
//!         Locale::Japanese => Response::simple("hello", "こんにちは世界"),
//!         _ => if let Some(ref s) = req.slot_value("name") {
//!             Response::simple("hello", (String::from("hello ") + s).as_str())
//!         } else {
//!             Response::simple("hello", "hello world")
//!         },
//!     };
//!     Ok(res)
//! }
//!
//! fn handle_cancel(_req: &Request) -> Result<Response,Error> {
//!     Ok(Response::end())
//! }
//!
//! async fn my_handler(event: LambdaEvent<Request>) -> Result<Response,Error> {
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

pub use self::request::Request;
pub use self::response::Response;
