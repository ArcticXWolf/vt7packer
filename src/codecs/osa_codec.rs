use std::fmt::Display;
use std::io::{Cursor, Read};

use serde::{Deserialize, Serialize};

use super::{Decoder, Encoder};
use crate::codecs;
use crate::error::DecodingError;
use crate::resource::Resource;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceItem {
    identifier: u32,
    filename: String,
}

#[derive(Debug)]
pub struct OsaCodec;

impl Display for OsaCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OsaCodec")
    }
}

impl Decoder for OsaCodec {
    fn matches_decoder(&self, resource: &Resource) -> usize {
        if resource.data.starts_with(&[0x41, 0x55, 0x46, 0x53]) {
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

        // AUFS header
        cursor.read_exact(&mut buffer)?;
        if buffer != [0x41, 0x55, 0x46, 0x53] {
            return Err(DecodingError::ParsingError(
                "Archive header is missing AUFS bytes".to_string(),
            ));
        }

        // Number of files header
        cursor.read_exact(&mut buffer)?;
        let number_of_files = u32::from_le_bytes(buffer);
        let mut directory_entry_buffer: [u8; 12] = [0; 12];

        // Parse directory and extract files
        for _ in 0..number_of_files {
            cursor.read_exact(&mut directory_entry_buffer)?;
            let mut res = Self::decode_single_resource(&resource, directory_entry_buffer)?;
            codecs::decode(&mut res)?;
            resource_items.push(ResourceItem {
                identifier: res.identifier,
                filename: res.get_filename(),
            });
            resources.push(res);
        }

        let serialized_lines = serde_json::to_string_pretty(&resource_items).unwrap();
        resource.data = serialized_lines.as_bytes().to_vec();
        resource.subresources = resources;
        resource.extension = Some("json".to_string());
        resource.format = Some("osa".to_string());
        Ok(())
    }
}

impl OsaCodec {
    fn decode_single_resource(
        resource: &Resource,
        directory_entry: [u8; 12],
    ) -> Result<Resource, DecodingError> {
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

        // Length
        directory_entry_cursor.read_exact(&mut buffer)?;
        let length = u32::from_le_bytes(buffer);

        // Read data
        resource_cursor.set_position(offset as u64);
        resource_cursor
            .take(length as u64)
            .read_to_end(&mut subresource.data)?;

        Ok(subresource)
    }
}

impl Encoder for OsaCodec {
    fn matches_encoder(&self, resource: &Resource) -> usize {
        if resource.extension.as_deref() == Some("json")
            && resource.format.as_deref() == Some("osa")
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
            let resource_path = resource_dirpath.join(resource_item.filename);

            let mut subresource = Resource::load_from(&resource_path)?;
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
        data.extend_from_slice(&[0x41, 0x55, 0x46, 0x53]);
        data.extend_from_slice(&u32::to_le_bytes(resource.subresources.len() as u32));

        let directory_offset = (8 + resource.subresources.len() * 12) as u32;

        let mut subresources_data: Vec<u8> = vec![];
        for mapper_entry in mapper {
            if let Some(subresource) = resource
                .subresources
                .iter()
                .find(|r| r.identifier == mapper_entry.identifier)
            {
                let subresource_data = subresource.data.clone();

                // Directory entry
                data.extend_from_slice(&u32::to_le_bytes(subresource.identifier));
                data.extend_from_slice(&u32::to_le_bytes(
                    directory_offset + subresources_data.len() as u32,
                ));
                data.extend_from_slice(&u32::to_le_bytes(subresource.data.len() as u32));

                subresources_data.extend(subresource_data);
            }
        }

        data.extend(subresources_data);
        resource.data = data;
        resource.extension = Some("osa".to_string());
        resource.format = None;
        resource.subresources.clear();

        Ok(())
    }
}
