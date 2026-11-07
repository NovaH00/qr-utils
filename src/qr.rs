use std::io::Cursor;
use qrcode::QrCode;
use image::{Luma, ImageFormat, ImageReader};
use rqrr::PreparedImage;

use crate::image::PngImage;

pub fn encode_qr(data: &str) -> anyhow::Result<PngImage> {
    let code = QrCode::new(data)?;
    let image = code.render::<Luma<u8>>().build();

    let mut bytes = Cursor::new(Vec::<u8>::new());
    image.write_to(&mut bytes, ImageFormat::Png)?;

    Ok(bytes.into())
}

pub fn decode_qr(image: PngImage) -> anyhow::Result<Vec<String>> {
    let img = image::load_from_memory(&image.into_inner())?;
    let mut preprared = PreparedImage::prepare(img.to_luma8());
    let grids = preprared.detect_grids();

    Ok(grids
        .iter()
        .filter_map(|grid| grid.decode().ok())
        .map(|(_, content)| { content })
        .collect::<Vec<_>>()
    )
}
