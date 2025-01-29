mod ogg_codec;
mod old_engine_codec;
mod osa_codec;
mod raw_codec;
mod save_codec;
mod sword_text_codec;
mod ttf_codec;
mod txt_codec;
mod vt7a_codec;
mod webm_codec;
mod webp_codec;
mod xml_codec;

use crate::{
    error::{DecodingError, EncodingError},
    resource::Resource,
};
use log::debug;
use std::{fmt::Display, path::Path};

pub trait Decoder {
    fn matches_decoder(&self, resource: &Resource) -> usize;
    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError>;
}

pub trait Encoder {
    fn matches_encoder(&self, resource: &Resource) -> usize;
    fn load_subresources(&self, path: &Path, resource: &mut Resource) -> Result<(), EncodingError>;
    fn encode(&self, resource: &mut Resource) -> Result<(), EncodingError>;
}

pub trait Codec: Encoder + Decoder + Display {}
impl<T> Codec for T where T: Encoder + Decoder + Display {}

fn get_codecs() -> Vec<Box<dyn Codec>> {
    vec![
        Box::new(old_engine_codec::OldEngineCodec {}),
        Box::new(ogg_codec::OggCodec {}),
        Box::new(osa_codec::OsaCodec {}),
        Box::new(raw_codec::RawCodec {}),
        Box::new(save_codec::SaveCodec {}),
        Box::new(sword_text_codec::SwordTextCodec {}),
        Box::new(ttf_codec::TtfCodec {}),
        Box::new(txt_codec::TxtCodec {}),
        Box::new(vt7a_codec::Vt7aCodec {}),
        Box::new(webm_codec::WebmCodec {}),
        Box::new(webp_codec::WebpCodec {}),
        Box::new(xml_codec::XmlCodec {}),
    ]
}

pub fn decode(resource: &mut Resource) -> Result<(), DecodingError> {
    let decoders = get_codecs();
    let mut best_match = 0;
    let mut best_decoder = decoders.last().unwrap();

    for decoder in &decoders {
        let current_match = decoder.matches_decoder(resource);
        if current_match > best_match {
            best_match = current_match;
            best_decoder = decoder;
        }
    }

    debug!("{}: {}", best_decoder, resource);
    best_decoder.decode(resource)
}

pub fn load_subresources(path: &Path, resource: &mut Resource) -> Result<(), EncodingError> {
    let encoders = get_codecs();
    let mut best_match = 0;
    let mut best_encoder = encoders.last().unwrap();

    for encoder in &encoders {
        let current_match = encoder.matches_encoder(resource);
        if current_match > best_match {
            best_match = current_match;
            best_encoder = encoder;
        }
    }

    debug!("{}: {}", best_encoder, resource);
    best_encoder.load_subresources(path, resource)
}

pub fn encode(resource: &mut Resource) -> Result<(), EncodingError> {
    let encoders = get_codecs();
    let mut best_match = 0;
    let mut best_encoder = encoders.last().unwrap();

    for encoder in &encoders {
        let current_match = encoder.matches_encoder(resource);
        if current_match > best_match {
            best_match = current_match;
            best_encoder = encoder;
        }
    }

    debug!("{}: {}", best_encoder, resource);
    best_encoder.encode(resource)
}
