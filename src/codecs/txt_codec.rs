use core::str;
use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct TxtCodec;

impl Display for TxtCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TxtCodec")
    }
}

impl Decoder for TxtCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        match str::from_utf8(&resource.data) {
            Ok(_) => 20,
            _ => 0,
        }
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("txt".to_string());
        Ok(())
    }
}

impl Encoder for TxtCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("txt") {
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
