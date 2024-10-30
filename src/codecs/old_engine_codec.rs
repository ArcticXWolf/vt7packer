use core::str;
use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct OldEngineCodec;

impl Display for OldEngineCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OldEngineCodec")
    }
}

impl Decoder for OldEngineCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with("ChrTxt".as_bytes())
            || resource.data.starts_with("Script".as_bytes())
            || resource.data.starts_with("LyrIdx".as_bytes())
            || resource.data.starts_with("Compat".as_bytes())
            || resource.data.starts_with("Sprite".as_bytes())
        {
            return 100;
        }
        0
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("oldengine".to_string());
        let format_type = str::from_utf8(&resource.data[0..6])
            .expect("Decoder should never be called on a non-string header")
            .trim();
        let version = u16::from_le_bytes(
            resource
                .data
                .iter()
                .skip(6)
                .take(2)
                .cloned()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        );
        resource.format = Some(format!("{}{}", format_type, version));
        Ok(())
    }
}

impl Encoder for OldEngineCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("oldengine") {
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
