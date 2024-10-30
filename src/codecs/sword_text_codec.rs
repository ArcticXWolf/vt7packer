use core::str;
use std::fmt::Display;
use std::io::{BufRead, Cursor, Read};

use serde::{Deserialize, Serialize};

use super::{Decoder, Encoder};
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextLine {
    identifier: u32,
    offset: u32,
    text: String,
}

#[derive(Debug)]
pub struct SwordTextCodec;

impl Display for SwordTextCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SwordTextCodec")
    }
}

impl Decoder for SwordTextCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with(&[0x54, 0x45, 0x58, 0x54]) {
            return 100;
        }
        0
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        let mut lines: Vec<TextLine> = vec![];

        let mut directory_cursor = Cursor::new(&resource.data);
        let mut text_cursor = Cursor::new(&resource.data);
        let mut field_buffer: [u8; 4] = [0; 4];

        // Skip header
        directory_cursor.set_position(8);

        // Extract amount of lines
        directory_cursor.read_exact(&mut field_buffer)?;
        let number_of_lines = u32::from_le_bytes(field_buffer);

        for _ in 0..number_of_lines {
            directory_cursor.read_exact(&mut field_buffer)?;
            let identifier = u32::from_le_bytes(field_buffer);

            directory_cursor.read_exact(&mut field_buffer)?;
            let offset = u32::from_le_bytes(field_buffer);

            let mut string_buffer = Vec::new();
            text_cursor.set_position(offset as u64);
            text_cursor.read_until(b'\0', &mut string_buffer)?;
            let text = str::from_utf8(&string_buffer).map_err(|e| {
                DecodingError::ParsingError(format!(
                    "Parsing text error with identifier {:08x} at offset 0x{:08x}: {}",
                    identifier, offset, e
                ))
            })?;

            lines.push(TextLine {
                identifier,
                offset,
                text: text.trim_matches(char::from(0)).to_string(),
            });
        }

        let serialized_lines = serde_json::to_string_pretty(&lines).unwrap();
        resource.data = serialized_lines.as_bytes().to_vec();
        resource.extension = Some("json".to_string());
        resource.format = Some("sword_text".to_string());

        Ok(())
    }
}

impl Encoder for SwordTextCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("json")
            && resource.format.as_deref() == Some("sword_text")
        {
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

    fn encode(&self, resource: &mut Resource) -> Result<(), crate::error::EncodingError> {
        let lines: Vec<TextLine> = serde_json::from_slice(&resource.data)?;
        let mut data: Vec<u8> = vec![];

        // Header
        data.extend_from_slice(&[0x54, 0x45, 0x58, 0x54, 0x00, 0x00, 0x00, 0x00]);
        data.extend_from_slice(&u32::to_le_bytes(lines.len() as u32));

        let directory_offset = (12 + lines.len() * 8) as u32;
        let mut lines_data: Vec<u8> = vec![];

        for line in lines {
            // directory entry
            data.extend_from_slice(&u32::to_le_bytes(line.identifier));
            data.extend_from_slice(&u32::to_le_bytes(
                directory_offset + lines_data.len() as u32,
            ));

            // data
            lines_data.extend(line.text.bytes());
            lines_data.push(char::from(0) as u8); // Null terminate the string
        }

        // append data
        data.extend(lines_data);

        resource.data = data;
        resource.extension = Some("swordtext".to_string());
        resource.format = None;
        resource.subresources.clear();

        Ok(())
    }
}
