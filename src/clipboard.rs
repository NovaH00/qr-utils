use std::process::{Command, Stdio};
use std::io::{Write, Read};
use crate::image::PngImage;
use anyhow::{Context};

pub fn copy_image_to_clipboard(img: PngImage) -> anyhow::Result<()> {
    let bytes = img.into_inner();

    let mut cmd = Command::new("xclip")
        .args([
            "-selection", "clipboard",
            "-target", "image/png"
        ])
        .stdin(Stdio::piped())
        .spawn()?;

    let mut stdin = cmd
        .stdin
        .take()
        .context("Failed to acquire xclip stdin")?;


    stdin.write_all(&bytes)?;

    Ok(())
}

pub fn retrieve_image_from_clipboard() -> anyhow::Result<PngImage> {
    let mut cmd = Command::new("xclip")
        .args([
            "-selection", "clipboard",
            "-target", "image/png",
            "-o"
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdout = cmd
        .stdout
        .take()
        .context("Failed to acquire xclip stdout")?;

    let mut bytes: Vec<u8> = vec![];
    stdout.read_to_end(&mut bytes)?;

    Ok(PngImage::new(bytes))
}
