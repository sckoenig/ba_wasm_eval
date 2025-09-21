//! Implements an image resize benchmark.
//! Creates a thumbnail version of a given image and saves it.
//!
//! Inspired by:
//! Vojdan Kjorveziroski & Sonja Filiposka 2023
//! https://github.com/korvoj/wasm-serverless-benchmarks/blob/master/functions/go/imageprocessing/main.go
//!
use bench::*;
use image::imageops::FilterType;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    benchmark(|| {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() != 1 {
            eprintln!("Usage: r_thumbnailer PATH/TO/IMAGE");
            return;
        }
        let filename = &args[0];

        if let Err(e) = thumbnail(filename) {
            eprintln!("Thumbnail error: {}", e);
        }
    });
}

/// Creates the path for the thumbnail image.
fn get_out_path(path: &str) -> PathBuf {
    let path = Path::new(path);
    let parent_dir = path.parent().unwrap_or(Path::new("."));
    // unwrap is safe here as the image was opened from that path without error, so the path is valid
    let filename = format!("r_{}_thumb_{}", timestamp(), path.file_name().unwrap().to_str().unwrap());

    parent_dir.join(filename)
}

/// Creates a thumbnail variant of a given image and saves it.
fn thumbnail(path: &String) -> image::ImageResult<()> {
    let img = image::open(path)?;
    let resized = img.resize_to_fill(150, 150, FilterType::Lanczos3);
    resized.save(get_out_path(path))?;

    Ok(())
}
