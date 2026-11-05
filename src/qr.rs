use std::io::Cursor;
use std::io::Write;
use std::fs::File;
use qrcode::QrCode;
use image::{Luma, ImageFormat};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PngImage(Vec<u8>);

impl PngImage {
    pub fn save(&self, file_path: impl Into<PathBuf>) -> anyhow::Result<()> {
        let file_path = file_path.into();
        let mut f = File::create(file_path)?;
        f.write_all(&self.0)?;

        Ok(())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<Cursor<Vec<u8>>> for PngImage {
    fn from(value: Cursor<Vec<u8>>) -> Self {
        PngImage(value.into_inner())
    }
}

pub fn encode_qr(data: &[u8]) -> anyhow::Result<PngImage> {
    let code = QrCode::new(data)?;
    let image = code.render::<Luma<u8>>().build();

    let mut bytes = Cursor::new(Vec::<u8>::new());
    image.write_to(&mut bytes, ImageFormat::Png)?;

    Ok(bytes.into())
}
