use std::{fmt::Display, fs, io, path::Path};

use sha2::{Digest, Sha256};

#[derive(Debug, Default, Clone)]
pub struct Resource {
    pub identifier: u32,
    pub format: Option<String>,
    pub extension: Option<String>,
    pub data: Vec<u8>,
    pub hidden: bool,
    pub subresources: Vec<Self>,
}

impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource #{:08x} (", self.identifier,)?;
        if let Some(x) = &self.format {
            write!(f, "Format: {}, ", x)?;
        }
        if let Some(x) = &self.extension {
            write!(f, "Extension: {}, ", x)?;
        }
        write!(f, "Children: {})", self.subresources.len())
    }
}

impl Resource {
    pub fn get_filename(&self) -> String {
        let mut filename = format!("{:08x}", self.identifier);

        if let Some(tag) = &self.format {
            filename = format!("{}.{}", filename, tag);
        }

        if let Some(extension) = &self.extension {
            filename = format!("{}.{}", filename, extension);
        }
        filename
    }

    pub fn get_dirname(&self) -> String {
        format!("{}.d", self.get_filename())
    }

    pub fn save(&self, path: &Path, save_hidden: bool) -> Result<(), io::Error> {
        if !save_hidden && self.hidden {
            return Ok(());
        }

        fs::create_dir_all(path)?;
        fs::write(path.join(self.get_filename()), &self.data)?;
        if self.subresources.len() > 0 {
            let dpath = path.join(self.get_dirname());
            fs::create_dir_all(&dpath)?;
            for resource in &self.subresources {
                resource.save(&dpath, save_hidden)?;
            }
        }

        Ok(())
    }

    pub fn parse_filename(&mut self, filename: &str) -> Result<(), io::Error> {
        let parts: Vec<&str> = filename.split('.').collect();

        match parts.len() {
            3 => {
                self.identifier =
                    u32::from_str_radix(parts.iter().nth(0).unwrap(), 16).map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "Filename does not contain identifier")
                    })?;
                self.format = Some(parts.iter().nth(1).unwrap().to_string());
                self.extension = Some(parts.iter().nth(2).unwrap().to_string());
            }
            2 => {
                self.identifier =
                    u32::from_str_radix(parts.iter().nth(0).unwrap(), 16).map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "Filename does not contain identifier")
                    })?;
                self.extension = Some(parts.iter().nth(1).unwrap().to_string());
            }
            1 => {
                self.identifier =
                    u32::from_str_radix(parts.iter().nth(0).unwrap(), 16).map_err(|_| {
                        io::Error::new(io::ErrorKind::Other, "Filename does not contain identifier")
                    })?;
            }
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Wrong filename")),
        }
        Ok(())
    }

    pub fn load_from(path: &Path) -> Result<Self, io::Error> {
        if !path.is_file() {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        let mut resource = Resource::default();

        let filename: &str = path.file_name().unwrap().to_str().unwrap();

        resource.parse_filename(filename)?;

        resource.data = fs::read(path)?;

        Ok(resource)
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let result = hasher.finalize();
        result.to_vec()
    }
}
