use flate2::read::GzDecoder;
use std::fs::File; // todo change to tokio fs
use tar::Archive;

/// unzip a gzip file, like tar -xvf does on posix based systems
pub fn tar_xvf(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    // Open the gzip file
    let file = File::open(filename)?;
    let decoder = GzDecoder::new(file);

    // Create a tar archive from the gzip file decoder
    let mut archive = Archive::new(decoder);

    // Extract the contents of the archive
    archive.unpack(".")?;
    Ok(())
}
