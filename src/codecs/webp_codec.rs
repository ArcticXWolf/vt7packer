use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct WebpCodec;

impl Display for WebpCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebpCodec")
    }
}

impl Decoder for WebpCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with(&[0x52, 0x49, 0x46, 0x46])
            && resource
                .data
                .iter()
                .skip(8)
                .take(4)
                .eq(&[0x57, 0x45, 0x42, 0x50])
        {
            return 100;
        }
        0
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("webp".to_string());
        Ok(())
    }
}

impl Encoder for WebpCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("webp") {
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
