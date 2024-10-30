use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct WebmCodec;

impl Display for WebmCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebmCodec")
    }
}

impl Decoder for WebmCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
            return 100;
        }
        0
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("webm".to_string());
        Ok(())
    }
}

impl Encoder for WebmCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("webm") {
            return 100;
        }
        0
    }

    fn load_subresources(
        &self,
        _path: &std::path::Path,
        _resource: &mut Resource,
    ) -> Result<(), crate::error::EncodingError> {
        Ok(())
    }

    fn encode(&self, _resource: &mut Resource) -> Result<(), crate::error::EncodingError> {
        Ok(())
    }
}
