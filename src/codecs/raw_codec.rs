use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct RawCodec;

impl Display for RawCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawCodec")
    }
}

impl Decoder for RawCodec {
    fn matches_decoder(&self, _resource: &Resource) -> usize {
        10
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("raw".to_string());
        resource.hidden = true;

        Ok(())
    }
}

impl Encoder for RawCodec {
    fn matches_encoder(&self, _resource: &Resource) -> usize {
        10
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
