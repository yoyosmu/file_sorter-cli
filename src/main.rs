use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Simple folder sorter",
    long_about = "A simple CLI tool that sorts files in a folder by type"
)]
struct Args {
    folder: Option<String>,

    #[arg(long)]
    watch: bool,

    #[arg(long)]
    dry_run: bool,
}

#[derive(Deserialize)]
struct Config {
    features: Features,
}

#[derive(Deserialize)]
struct Features {
    images: bool,
    documents: bool,
    audio: bool,
    video: bool,
    archives: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            images: true,
            documents: true,
            audio: true,
            video: true,
            archives: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            features: Features {
                images: true,
                documents: true,
                audio: true,
                video: true,
                archives: true,
            },
        }
    }
}

const DEFAULT_CONFIG: &str = r#"
[features]
images = true
documents = true
audio = true
video = true
archives = true
"#;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.watch {
        loop {
            run_once(&args)?;
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        run_once(&args)?;
    }

    Ok(())
}

fn run_once(args: &Args) -> std::io::Result<()> {
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

    let images = [
        "jpg", "png", "webp", "jpeg", "gif", "avif", "tiff", "bmp", "raw", "heif", "heic",
    ];
    let docs = [
        "docx", "pdf", "doc", "odt", "txt", "rtf", "xps", "xlsx", "csv", "ods",
    ];
    let audio = [
        "mp3", "wav", "aac", "flac", "ogg", "wma", "aiff", "m4a", "dts", "opus",
    ];
    let video = [
        "mp4", "avi", "mov", "webm", "flv", "wmv", "mpg", "mpeg", "3gp", "3g2", "m4v", "mkv",
    ];
    let archive = [
        "zip", "tar", "gz", "bz2", "7z", "rar", "xz", "lzh", "lha", "taz", "pkg", "deb", "tgz",
        "lzip",
    ];

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

                let dest = if config.features.documents && docs.contains(&ext.as_str()) {
                    docs_count += 1;
                    fs::create_dir_all(&pdf_dir)?;
                    pdf_dir.join(file_name)
                } else if config.features.images && images.contains(&ext.as_str()) {
                    images_count += 1;
                    fs::create_dir_all(&img_dir)?;
                    img_dir.join(file_name)
                } else if config.features.audio && audio.contains(&ext.as_str()) {
                    audio_count += 1;
                    fs::create_dir_all(&audio_dir)?;
                    audio_dir.join(file_name)
                } else if config.features.video && video.contains(&ext.as_str()) {
                    video_count += 1;
                    fs::create_dir_all(&video_dir)?;
                    video_dir.join(file_name)
                } else if config.features.archives && archive.contains(&ext.as_str()) {
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
