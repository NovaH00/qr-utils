use std::process::ExitCode;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use anyhow::{Context};
use image::{ImageReader, ImageFormat};
use clap::{Parser, Subcommand};
use qr_utils::qr::{encode_qr, decode_qr};
use qr_utils::image::PngImage;
use qr_utils::clipboard::{
    copy_image_to_clipboard,
    retrieve_image_from_clipboard
};

#[derive(Debug, Parser)]
#[command(version, about = "QR code tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate QR code from provided data, and copy to clipboard as a PNG image.
    /// When save_path is set, the QR code also saved to that directory.
    Gen {
        /// The directory to save the generated QR code image.
        #[arg(short, long, required = false)]
        save_path: Option<PathBuf>,

        /// The data used to generate the QR code.
        data: Vec<String>
    },
    /// Scan the image in clipboard for QR codes.
    /// When image_path is set, it will scan the image_path instead.
    Scan {
        /// The image path to scan for QR codes.
        #[arg(short, long, required = false)]
        image_path: Option<PathBuf>
    }
}

fn handle_qr_generation(
    save_path: Option<PathBuf>,
    data: Vec<String>
) -> ExitCode {

    let data = data.join(" ");
    let png = match encode_qr(&data) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Could not encode the data: {e:#}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = copy_image_to_clipboard(png.clone()) {
        eprintln!("Could not copy the QR to clipboard: {e:#}");
        return ExitCode::FAILURE;

    } else {
        println!("QR code copied to clipboard!");
    }

    if let Some(path) = save_path {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let file_name = format!("qr_code_{millis}.png");
        let full_file_path = path.join(file_name);
        if let Err(e) = png.save(&full_file_path) {
            eprintln!(
                "Could not save the QR to {}: {e:#}",
                path.display()
            );
            return ExitCode::FAILURE;

        } else {
            println!(
                "QR code save to {}!",
                full_file_path.display()
            );
        }
    }

    ExitCode::SUCCESS
}

fn get_png_image(image_path: Option<PathBuf>) -> anyhow::Result<PngImage> {
    match image_path {
        Some(path) => {
            let img = ImageReader::open(&path)
                .context(format!("Could not open {}:", path.display()))?
                .decode()
                .context(format!("Could not decode image {}", path.display()))?;

            let mut bytes = Cursor::new(Vec::<u8>::new());
            img.write_to(&mut bytes, ImageFormat::Png)
                .context(format!("Could not write image {} to buffer", path.display()))?;

            Ok(bytes.into())
        },
        None => retrieve_image_from_clipboard()
            .context(format!("Could not retrieve image from clipboard"))
    }
}

fn handle_qr_scan(image_path: Option<PathBuf>) -> ExitCode {
    let img = match get_png_image(image_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("{e:#}");
            return ExitCode::FAILURE;
        }
    };

    let results = match decode_qr(img) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Could not decode the QR: {e:#}");
            return ExitCode::FAILURE;
        }
    };

    println!("Detected {} QR(s)", results.len());

    for (index, data) in results.iter().enumerate() {
        println!("{}: \"{}\"", index + 1, data);
    }

    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args = Cli::parse();
    match args.command {
        Commands::Gen { save_path, data }
            => handle_qr_generation(save_path, data),
        Commands::Scan { image_path }
            => handle_qr_scan(image_path)
    }
}
