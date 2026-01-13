use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn scan_images(folder: &str) -> Vec<PathBuf> {
    WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && matches!(
                    e.path().extension().and_then(|s| s.to_str()),
                    Some("jpg" | "jpeg" | "png")
                )
        })
        .map(|e| e.path().to_owned())
        .collect()
}

pub fn embed_tags_metadata(path: &Path, tags: &[String]) -> anyhow::Result<()> {
    // Use exiftool to write tags.
    // -overwrite_original: don't create _original backup files
    // -Keywords: standard iptc/legacy
    // -Subject: XMP
    // -XPKeywords: Windows
    // -Comment: simple comment
    let tags_str = tags.join(", ");
    let output = Command::new("exiftool")
        .arg("-overwrite_original")
        .arg(format!("-Keywords={}", tags_str))
        .arg(format!("-Subject={}", tags_str))
        .arg(format!("-XPKeywords={}", tags_str))
        .arg(format!("-Comment={}", tags_str))
        .arg("-UserComment=local_lens_processed")
        .arg(path)
        .output();

    match output {
        Ok(out) => {
            if !out.status.success() {
                let err = String::from_utf8_lossy(&out.stderr);
                anyhow::bail!("Exiftool failed: {}", err);
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow::bail!(
                    "exiftool not found. Please install it: sudo apt install libimage-exiftool-perl"
                );
            }
            return Err(e.into());
        }
    }
    Ok(())
}

pub fn is_already_tagged(path: &Path) -> bool {
    // Check if UserComment contains "local_lens_processed"
    // false
    let output = Command::new("exiftool")
        .arg("-UserComment")
        .arg("-b")
        .arg(path)
        .output();

    if let Ok(out) = output {
        let comment = String::from_utf8_lossy(&out.stdout);
        return comment.contains("local_lens_processed");
    }
    false
}
