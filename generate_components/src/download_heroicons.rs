use anyhow::Result;
use reqwest::blocking::Client;
use std::fs;
use std::io;
use std::path;

use crate::constants::*;

pub fn download_latest_icons() -> Result<()> {
    let client = Client::new();
    let mut response = client.get(HEROICONS_ARCHIVE_URL).send()?;

    let mut zip_file = fs::File::create(ZIP_FILE)?;
    io::copy(&mut response, &mut zip_file)?;

    Ok(())
}

/// Extracts the zip archive.
/// See: https://github.com/zip-rs/zip/blob/master/examples/extract.rs
pub fn extract_latest_icons() -> Result<()> {
    let zip_file_name = path::Path::new(ZIP_FILE);
    let zip_file = fs::File::open(zip_file_name).unwrap();

    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // We only need the optimized files.
        if !outpath.starts_with(OPTIMIZED_PATH) {
            continue;
        }

        println!("Extracting file {outpath:?}");

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                    .unwrap();
            }
        }
    }

    fs::remove_file(zip_file_name)?;

    Ok(())
}
