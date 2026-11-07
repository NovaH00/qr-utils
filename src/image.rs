use std::io::Write;
use std::io::Cursor;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct PngImage(Vec<u8>);


impl PngImage {
    pub fn new(data: impl Into<Vec<u8>>) -> Self {
        Self(data.into())
    }

    pub fn save(
        &self,
        file_path: impl Into<PathBuf>
    ) -> anyhow::Result<()> {
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

