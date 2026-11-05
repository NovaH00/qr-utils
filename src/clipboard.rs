use std::process::{Command, Stdio};
use std::io::Write;
use crate::qr::PngImage;
use anyhow::{Context};

pub fn copy_image_to_clipboard(img: PngImage) -> anyhow::Result<()> {
    let bytes = img.into_inner();

    let mut xclip = Command::new("xclip")
        .args([
            "-selection", "clipboard",
            "-target", "image/png"
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = xclip
        .stdin
        .take()
        .context("Failed to acquire xclip stdin")?;


    stdin.write_all(&bytes)?;

    Ok(())
}
