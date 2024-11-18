use std::{collections::HashMap, fs, io::Read, path::PathBuf};

use crate::{codecs, resource::Resource};

pub fn decode(
    filepath: &PathBuf,
    outpath: &PathBuf,
    save_hidden: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open(filepath)?;
    let mut archive = Resource::default();
    file.read_to_end(&mut archive.data)?;
    codecs::decode(&mut archive)?;
    archive.save(&outpath, save_hidden)?;
    log::info!(
        "Unpacked files to: {}",
        &outpath.join(archive.get_filename()).to_string_lossy()
    );
    Ok(())
}

pub fn statistics(filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::open(filepath)?;
    let mut archive = Resource::default();
    file.read_to_end(&mut archive.data)?;
    codecs::decode(&mut archive)?;
    let mut map: HashMap<(Option<String>, Option<String>), usize> = HashMap::new();
    for res in archive.subresources {
        let values = map
            .entry((res.extension.clone(), res.format.clone()))
            .or_default();
        *values += 1;
    }
    log::info!("{:>10} | {:>10} | {:>5}", "Extension", "Format", "Amount");
    for ((extension, format), v) in map {
        log::info!(
            "{:>10} | {:>10} | {:>5}",
            extension.as_deref().unwrap_or_default(),
            format.as_deref().unwrap_or_default(),
            v
        );
    }
    Ok(())
}

pub fn diff(filepath1: &PathBuf, filepath2: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut file1 = fs::File::open(filepath1)?;
    let mut archive1 = Resource::default();
    file1.read_to_end(&mut archive1.data)?;
    codecs::decode(&mut archive1)?;

    let mut file2 = fs::File::open(filepath2)?;
    let mut archive2 = Resource::default();
    file2.read_to_end(&mut archive2.data)?;
    codecs::decode(&mut archive2)?;

    log::info!("Files only in {}:", filepath1.display());
    for r1 in &archive1.subresources {
        if archive2
            .subresources
            .iter()
            .find(|r2| r2.identifier == r1.identifier)
            .is_none()
        {
            log::info!("  - {}", r1.get_filename());
        }
    }
    log::info!("");

    log::info!("Files only in {}:", filepath2.display());
    for r2 in &archive2.subresources {
        if archive1
            .subresources
            .iter()
            .find(|r1| r2.identifier == r1.identifier)
            .is_none()
        {
            log::info!("  - {}", r2.get_filename());
        }
    }
    log::info!("");

    log::info!("Files present in both, but changed:");
    for r2 in &archive2.subresources {
        if let Some(r1) = archive1
            .subresources
            .iter()
            .find(|r1| r2.identifier == r1.identifier)
        {
            if r1.hash() != r2.hash() {
                log::info!("  - {}", r2.get_filename());
            }
        }
    }
    Ok(())
}

pub fn encode(filepath: &PathBuf, outpath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut archive = Resource::load_from(&filepath)?;
    codecs::load_subresources(&filepath.parent().unwrap(), &mut archive)?;
    codecs::encode(&mut archive)?;
    archive.save(&outpath, true)?;
    log::info!(
        "Packed files to: {}",
        &outpath.join(archive.get_filename()).to_string_lossy()
    );
    Ok(())
}
