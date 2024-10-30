use std::fmt::Display;
use std::io::{self, Cursor, Read};

use serde::{Deserialize, Serialize};
use serde_with::base64::Base64;
use serde_with::serde_as;

use super::{Decoder, Encoder};
use crate::codecs;
use crate::error::{DecodingError, EncodingError};
use crate::resource::Resource;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceItem {
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

        // Version header
        cursor.read_exact(&mut buffer)?;
        if buffer != [0x02, 0x00, 0x00, 0x00] {
            return Err(DecodingError::ParsingError(format!(
                "Archive header has wrong version number {:?}",
                buffer
            )));
        }

        // Unknown1 header
        cursor.read_exact(&mut buffer)?;
        resource.identifier = u32::from_le_bytes(buffer);

        // Number of files header
        cursor.read_exact(&mut buffer)?;
        let number_of_files = u32::from_le_bytes(buffer);
        let mut directory_entry_buffer: [u8; 16] = [0; 16];

        // Parse directory and extract files
        for _ in 0..number_of_files {
            cursor.read_exact(&mut directory_entry_buffer)?;
            let (mut res, compressed) =
                Self::decode_single_resource(&resource, directory_entry_buffer)?;
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
        resource.format = Some("vt7a".to_string());
        Ok(())
    }
}

impl Vt7aCodec {
    fn decode_single_resource(
        resource: &Resource,
        directory_entry: [u8; 16],
    ) -> Result<(Resource, bool), DecodingError> {
        let mut subresource = Resource::default();
        let mut resource_cursor = Cursor::new(&resource.data);
        let mut directory_entry_cursor = Cursor::new(&directory_entry);
        let mut buffer: [u8; 4] = [0; 4];

        // Identifier
        directory_entry_cursor.read_exact(&mut buffer)?;
        let identifier = u32::from_le_bytes(buffer);
        subresource.identifier = identifier;

        // Offset
        directory_entry_cursor.read_exact(&mut buffer)?;
        let offset = u32::from_le_bytes(buffer);

        // Uncompressed size
        directory_entry_cursor.read_exact(&mut buffer)?;
        let size_uncompressed = u32::from_le_bytes(buffer);

        // Compressed size
        directory_entry_cursor.read_exact(&mut buffer)?;
        let size_compressed = u32::from_le_bytes(buffer);

        // Read data
        resource_cursor.set_position(offset as u64);
        match size_compressed {
            0 => {
                resource_cursor
                    .take(size_uncompressed as u64)
                    .read_to_end(&mut subresource.data)?;
            }
            _ => {
                let mut data = vec![];
                resource_cursor
                    .take(size_compressed as u64)
                    .read_to_end(&mut data)?;

                subresource.data = miniz_oxide::inflate::decompress_to_vec_zlib(&mut data)?;
            }
        };

        Ok((subresource, size_compressed != 0))
    }
}

impl Encoder for Vt7aCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("json")
            && resource.format.as_deref() == Some("vt7a")
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
        // first encode all subresources
        for mut subresource in resource.subresources.iter_mut() {
            codecs::encode(&mut subresource)?;
        }

        let mut data: Vec<u8> = vec![];
        let mapper: Vec<ResourceItem> = serde_json::from_slice(&resource.data)?;

        // Header
        data.extend_from_slice(&[0x56, 0x54, 0x37, 0x41]);
        data.extend_from_slice(&[0x02, 0x00, 0x00, 0x00]);
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
                let subresource_data = if mapper_entry.compressed {
                    miniz_oxide::deflate::compress_to_vec_zlib(&subresource.data, 10)
                } else {
                    subresource.data.clone()
                };

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
