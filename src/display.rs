//! Alexa SDK Display interface data structures, from [the specification](https://developer.amazon.com/en-US/docs/alexa/custom-skills/display-interface-reference.html).
//! Note that the Display interface is deprecated, though some data structures are also used in non-deprecated interfaces
//! like [`super::audioplayer`].
//! 
//! ## Note on pixel types
//! We use u16 for pixel dimension values, based on the maximum values in the size chart from
//! [the specification](https://developer.amazon.com/en-US/docs/alexa/custom-skills/display-interface-reference.html#image-sizes).
//! 
//! The corresponding JSON type mapping is integer. So we are a bit more strict in our representation 
//! than the serialized wire format. This will not create any problems for serialization, but it does mean 
//! that a JSON value may fail to deserialize if it contains out-of-range or negative pixel values.

use serde::{Deserialize, Serialize};

use crate::declare_api_enum;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_description: Option<String>,

    pub sources: Vec<ImageInstance>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImageInstance {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ImageSize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_pixels: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_pixels: Option<u16>,
}

declare_api_enum! {
    ImageSize => "SCREAMING_SNAKE_CASE" {
        XSmall,
        Small,
        Medium,
        Large,
        XLarge
    }
}
