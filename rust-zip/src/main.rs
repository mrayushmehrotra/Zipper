use std::fs::File;
use std::io::{self};
use std::time::Instant;
use zip::{CompressionMethod, write::FileOptions, write::ZipWriter};

fn zip_source(source: &str, target: &str) -> io::Result<()> {
    let start_time = Instant::now(); // Record the start time

    let target_file = File::create(target)?;
    let mut zip = ZipWriter::new(target_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored); // or CompressionMethod::Deflated

    for entry in walkdir::WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();

        let rel_path = path
            .strip_prefix(source)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if path.is_dir() {
            zip.add_directory(rel_path.to_string_lossy(), options)?;
        } else {
            let mut file = File::open(path)?;
            zip.start_file(rel_path.to_string_lossy(), options)?;
            io::copy(&mut file, &mut zip)?;
        }
    }

    zip.finish()?;
    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    println!("Time taken to zip: {:?}", elapsed_time);

    Ok(())
}

fn main() -> io::Result<()> {
    let source = "video.mp4";
    let target = "zipped.zip";

    if let Err(err) = zip_source(source, target) {
        eprintln!("Error: {}", err);
    }

    Ok(())
}
