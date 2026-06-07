use crate::{Args, Config, DEFAULT_CONFIG};
use crate::extensions::{IMAGES, DOCS, AUDIO, VIDEO, ARCHIVE};
use std::fs;

pub fn main(args: &Args) -> std::io::Result<()> {
    let config_dir = dirs::config_dir().unwrap().join("file_sorter");
    let config_path = config_dir.join("Sorter.toml");

    let downloads = dirs::download_dir().unwrap_or_else(|| std::env::current_dir().unwrap());

    let folder = args
        .folder
        .clone()
        .map(std::path::PathBuf::from)
        .unwrap_or(downloads);

    let paths = fs::read_dir(&folder)?;

    let mut docs_count = 0;
    let mut images_count = 0;
    let mut audio_count = 0;
    let mut video_count = 0;
    let mut archive_count = 0;

    let pdf_dir = folder.join("PDFs");
    let img_dir = folder.join("IMGs");
    let audio_dir = folder.join("AUDIOs");
    let video_dir = folder.join("VIDs");
    let archive_dir = folder.join("ARCHIVEs");

    if !config_path.exists() {
        fs::create_dir_all(&config_dir)?;
        fs::write(&config_path, DEFAULT_CONFIG)?;
        println!("Created config: {:?}", config_path);
    }

    let config_text = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_text).unwrap_or_default();

    for entry in paths {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            let file_name = match path.file_name() {
                Some(name) => name,
                None => continue,
            };

            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();

                let dest = if config.features.documents && DOCS.contains(&ext.as_str()) {
                    docs_count += 1;
                    fs::create_dir_all(&pdf_dir)?;
                    pdf_dir.join(file_name)
                } else if config.features.images && IMAGES.contains(&ext.as_str()) {
                    images_count += 1;
                    fs::create_dir_all(&img_dir)?;
                    img_dir.join(file_name)
                } else if config.features.audio && AUDIO.contains(&ext.as_str()) {
                    audio_count += 1;
                    fs::create_dir_all(&audio_dir)?;
                    audio_dir.join(file_name)
                } else if config.features.video && VIDEO.contains(&ext.as_str()) {
                    video_count += 1;
                    fs::create_dir_all(&video_dir)?;
                    video_dir.join(file_name)
                } else if config.features.archives && ARCHIVE.contains(&ext.as_str()) {
                    archive_count += 1;
                    fs::create_dir_all(&archive_dir)?;
                    archive_dir.join(file_name)
                } else {
                    continue;
                };

                if args.dry_run {
                    println!(
                        "[DRY RUN] {:?} -> {:?}",
                        path.file_name().unwrap(),
                        dest.file_name().unwrap()
                    );
                } else {
                    fs::rename(&path, &dest)?;
                }
            }
        }
    }

    let total = docs_count + images_count + audio_count + video_count + archive_count;

    println!("Documents: {}", docs_count);
    println!("Images: {}", images_count);
    println!("Audio: {}", audio_count);
    println!("Video: {}", video_count);
    println!("Archives: {}", archive_count);
    println!("Total: {}", total);

    Ok(())
}
