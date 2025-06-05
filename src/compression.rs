use std::io::{Read, Write};

use crate::error::{DecodingError, EncodingError};

pub enum CompressionFormat {
    None,
    Zlib,
    Zstd,
}

pub fn decompress(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>, DecodingError> {
    match format {
        CompressionFormat::None => decompress_none(data),
        CompressionFormat::Zlib => decompress_zlib(data),
        CompressionFormat::Zstd => decompress_zstd(data),
    }
}

fn decompress_none(data: &[u8]) -> Result<Vec<u8>, DecodingError> {
    Ok(data.to_vec())
}

fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>, DecodingError> {
    let mut buf = vec![];
    let mut decoder = flate2::read::ZlibDecoder::new(data);
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, DecodingError> {
    let mut buf = vec![];
    let mut decoder = zstd::Decoder::new(data)?;
    decoder.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn compress(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>, EncodingError> {
    match format {
        CompressionFormat::None => compress_none(data),
        CompressionFormat::Zlib => compress_zlib(data),
        CompressionFormat::Zstd => compress_zstd(data),
    }
}

fn compress_none(data: &[u8]) -> Result<Vec<u8>, EncodingError> {
    Ok(data.to_vec())
}

fn compress_zlib(data: &[u8]) -> Result<Vec<u8>, EncodingError> {
    let mut buf = vec![];
    let mut encoder = flate2::read::ZlibEncoder::new(data, flate2::Compression::best());
    encoder.read_to_end(&mut buf)?;
    Ok(buf)
}

fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, EncodingError> {
    let buf = vec![];
    let mut encoder = zstd::Encoder::new(buf, 0)?;
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}
