use std::fmt::Display;

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug)]
pub struct XmlCodec;

impl Display for XmlCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XmlCodec")
    }
}

impl Decoder for XmlCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        let reader = xml::EventReader::new(&resource.data[..]);
        for e in reader {
            if e.is_err() {
                return 0;
            }
        }
        return 100;
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        resource.extension = Some("xml".to_string());
        Ok(())
    }
}

impl Encoder for XmlCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("xml") {
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
