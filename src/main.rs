use clap::Parser;
use serde::Deserialize;
use std::thread;
use std::time::Duration;
mod sorter;
mod extensions;

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
            sorter::main(&args)?;
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        sorter::main(&args)?;
    }

    Ok(())
}
