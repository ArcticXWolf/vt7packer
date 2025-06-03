use std::fmt::Display;
use std::io::{self, Cursor, Read};

use log::trace;
use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

use super::{Decoder, Encoder};
use crate::codecs;
use crate::compression::{compress, decompress, CompressionFormat};
use crate::error::{DecodingError, EncodingError};
use crate::resource::Resource;

#[derive(Debug, Clone, Copy)]
pub enum Vt7aVersion {
    Two,
    Three,
}

impl TryFrom<u32> for Vt7aVersion {
    type Error = DecodingError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(DecodingError::ParsingError(format!(
                "Archive header has wrong version number {}",
                value
            ))),
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
struct ResourceItem {
    identifier: u32,
    compressed: bool,
    filename: String,
    #[serde_as(as = "Option<Base64>")]
    original_data: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Vt7aCodec;

impl Display for Vt7aCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vt7aCodec")
    }
}

impl Decoder for Vt7aCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with(&[0x56, 0x54, 0x37, 0x41]) {
            return 100;
        }
        0
    }

    fn decode(&self, resource: &mut Resource) -> Result<(), DecodingError> {
        let mut resources: Vec<Resource> = vec![];
        let mut resource_items: Vec<ResourceItem> = vec![];
        let mut cursor = Cursor::new(&resource.data);
        let mut buffer: [u8; 4] = [0; 4];

        cursor.set_position(0);

        // VT7A header
        cursor.read_exact(&mut buffer)?;
        if buffer != [0x56, 0x54, 0x37, 0x41] {
            return Err(DecodingError::ParsingError(
                "Archive header is missing VT7A bytes".to_string(),
            ));
        }
        trace!("VT7A header: {:?}", buffer);

        // Version header
        cursor.read_exact(&mut buffer)?;
        let archive_version = Vt7aVersion::try_from(u32::from_le_bytes(buffer))?;
        trace!("VT7A Version: {:?}", archive_version);

        // Unknown1 header
        cursor.read_exact(&mut buffer)?;
        resource.identifier = u32::from_le_bytes(buffer);
        trace!("VT7A Identifier: {:?}", buffer);

        // Number of files header
        cursor.read_exact(&mut buffer)?;
        let number_of_files = u32::from_le_bytes(buffer);
        let mut directory_entry_buffer: [u8; 16] = [0; 16];
        trace!("VT7A NumOfFiles: {}", number_of_files);

        // Parse directory and extract files
        for _ in 0..number_of_files {
            cursor.read_exact(&mut directory_entry_buffer)?;
            let (mut res, compressed) =
                Self::decode_single_resource(resource, directory_entry_buffer, archive_version)?;
            codecs::decode(&mut res)?;
            resource_items.push(ResourceItem {
                identifier: res.identifier,
                compressed,
                filename: res.get_filename(),
                original_data: match res.extension.as_deref() {
                    Some("raw") => Some(res.data.clone()),
                    _ => None,
                },
            });
            resources.push(res);
        }

        let serialized_lines = serde_json::to_string_pretty(&resource_items).unwrap();
        resource.data = serialized_lines.as_bytes().to_vec();
        resource.subresources = resources;
        resource.extension = Some("json".to_string());
        resource.format = match archive_version {
            Vt7aVersion::Two => Some("vt7a2".to_string()),
            Vt7aVersion::Three => Some("vt7a3".to_string()),
        };
        Ok(())
    }
}

impl Vt7aCodec {
    fn decode_single_resource(
        resource: &Resource,
        directory_entry: [u8; 16],
        archive_version: Vt7aVersion,
    ) -> Result<(Resource, bool), DecodingError> {
        let mut subresource = Resource::default();
        let mut resource_cursor = Cursor::new(&resource.data);
        let mut directory_entry_cursor = Cursor::new(&directory_entry);
        let mut buffer: [u8; 4] = [0; 4];

        // Identifier
        directory_entry_cursor.read_exact(&mut buffer)?;
        let identifier = u32::from_le_bytes(buffer);
        subresource.identifier = identifier;
        trace!("-   VT7A File Identifier: {:#x}", identifier);

        // Offset
        directory_entry_cursor.read_exact(&mut buffer)?;
        let offset = u32::from_le_bytes(buffer);
        trace!("    VT7A File Offset: {:#x}", offset);

        // Uncompressed size
        directory_entry_cursor.read_exact(&mut buffer)?;
        let size_uncompressed = u32::from_le_bytes(buffer);
        trace!("    VT7A File Size Uncompressed: {:#x}", size_uncompressed);

        // Compressed size
        directory_entry_cursor.read_exact(&mut buffer)?;
        let size_compressed = u32::from_le_bytes(buffer);
        trace!("    VT7A File Size Compressed: {:#x}", size_compressed);

        // Read data
        resource_cursor.set_position(offset as u64);
        let mut size_to_read = size_uncompressed;
        let mut compression_format = CompressionFormat::None;
        if size_compressed != 0 {
            size_to_read = size_compressed;
            compression_format = match archive_version {
                Vt7aVersion::Two => CompressionFormat::Zlib,
                Vt7aVersion::Three => CompressionFormat::Zstd,
            };
        }
        let mut data = vec![];
        resource_cursor
            .take(size_to_read as u64)
            .read_to_end(&mut data)?;
        subresource.data = decompress(&data, compression_format)?;

        Ok((subresource, size_compressed != 0))
    }
}

impl Encoder for Vt7aCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("json")
            && (resource.format.as_deref() == Some("vt7a")
                || resource.format.as_deref() == Some("vt7a2")
                || resource.format.as_deref() == Some("vt7a3"))
        {
            return 100;
        }
        0
    }

    fn load_subresources(
        &self,
        path: &std::path::Path,
        resource: &mut Resource,
    ) -> Result<(), crate::error::EncodingError> {
        let mapper: Vec<ResourceItem> = serde_json::from_slice(&resource.data)?;
        let resource_dirpath = path.join(resource.get_dirname());

        for resource_item in mapper {
            let resource_path = resource_dirpath.join(&resource_item.filename);

            let mut subresource = match Resource::load_from(&resource_path) {
                Ok(v) => v,
                Err(_) => {
                    let mut r = Resource::default();
                    r.parse_filename(&resource_item.filename.clone())?;
                    r.data = match resource_item.original_data {
                        Some(d) => d,
                        None => {
                            return Err(EncodingError::IOError(io::Error::new(
                                io::ErrorKind::NotFound,
                                format!("File {} not found", &resource_item.filename),
                            )))
                        }
                    };
                    r
                }
            };
            codecs::load_subresources(&resource_dirpath, &mut subresource)?;
            resource.subresources.push(subresource);
        }

        Ok(())
    }

    fn encode(&self, resource: &mut Resource) -> Result<(), crate::error::EncodingError> {
        let archive_version = match resource.format.as_deref() {
            Some("vt7a") | Some("vt7a2") => Vt7aVersion::Two,
            Some("vt7a3") => Vt7aVersion::Three,
            _ => {
                return Err(EncodingError::ParsingError(
                    "wrong archive format".to_string(),
                ))
            }
        };

        for subresource in resource.subresources.iter_mut() {
            codecs::encode(subresource)?;
        }

        let mut data: Vec<u8> = vec![];
        let mapper: Vec<ResourceItem> = serde_json::from_slice(&resource.data)?;

        // Header
        data.extend_from_slice(&[0x56, 0x54, 0x37, 0x41]);
        match archive_version {
            Vt7aVersion::Two => data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]),
            Vt7aVersion::Three => data.extend_from_slice(&[0x03, 0x00, 0x00, 0x00]),
        };
        data.extend_from_slice(&u32::to_le_bytes(resource.identifier));
        data.extend_from_slice(&u32::to_le_bytes(resource.subresources.len() as u32));

        let directory_offset = (16 + resource.subresources.len() * 16) as u32;

        let mut subresources_data: Vec<u8> = vec![];
        for mapper_entry in mapper {
            if let Some(subresource) = resource
                .subresources
                .iter()
                .find(|r| r.identifier == mapper_entry.identifier)
            {
                let mut compression_format = CompressionFormat::None;
                if mapper_entry.compressed {
                    compression_format = match archive_version {
                        Vt7aVersion::Two => CompressionFormat::Zlib,
                        Vt7aVersion::Three => CompressionFormat::Zstd,
                    };
                }

                let subresource_data = compress(&subresource.data, compression_format)?;

                // Directory entry
                data.extend_from_slice(&u32::to_le_bytes(subresource.identifier));
                data.extend_from_slice(&u32::to_le_bytes(
                    directory_offset + subresources_data.len() as u32,
                ));
                data.extend_from_slice(&u32::to_le_bytes(subresource.data.len() as u32));
                if mapper_entry.compressed {
                    data.extend_from_slice(&u32::to_le_bytes(subresource_data.len() as u32));
                } else {
                    data.extend_from_slice(&u32::to_le_bytes(0));
                }

                subresources_data.extend(subresource_data);
            }
        }

        data.extend(subresources_data);
        resource.data = data;
        resource.extension = Some("vt7a".to_string());
        resource.format = None;
        resource.subresources.clear();

        Ok(())
    }
}
