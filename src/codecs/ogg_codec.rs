use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct OggCodec;

impl Display for OggCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OggCodec")
    }
}

impl Decoder for OggCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with(&[0x4f, 0x67, 0x67, 0x53]) {
            return 100;
        }
        0
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("ogg".to_string());
        Ok(())
    }
}

impl Encoder for OggCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("ogg") {
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
