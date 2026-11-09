use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use clap::Parser;
use qr_utils::qr::{encode_qr, decode_qr};
use qr_utils::clipboard::{
    copy_image_to_clipboard,
    retrieve_image_from_clipboard
};

#[derive(Parser, Debug)]
#[command(version, about = "Generate QR code quickly")]
struct Cli {
    /// Switch to scan mode that scan the image in clipboard for QR code
    #[arg(short, long)]
    scan: bool,

    #[arg(long, required_unless_present = "scan")]
    save_path: Option<PathBuf>,

    /// The data to convert to QR code
    #[arg(required_unless_present = "scan")]
    data: Vec<String>,
}

fn main() -> ExitCode {
    let args = Cli::parse();

    if !args.scan {
        let data = args.data.join(" ");
        let png = match encode_qr(&data) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Could not encode the data: {e}");
                return ExitCode::FAILURE;
            }
        };

        if let Err(e) = copy_image_to_clipboard(png.clone()) {
            eprintln!("Could not copy the QR to clipboard: {}", e);
            return ExitCode::FAILURE;
        } else {
            println!("QR code copied to clipboard!");
        }

        if let Some(path) = args.save_path {
            let millis = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            let file_name = format!("qr_code_{millis}.png");
            let full_file_path = path.join(file_name);
            if let Err(e) = png.save(&full_file_path) {
                eprintln!(
                    "Could not save the QR to {}: {e}",
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
    } else {
        let img = match retrieve_image_from_clipboard() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Could not retrieve image from clipboard: {e}");
                return ExitCode::FAILURE;
            }
        };

        let results = match decode_qr(img) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("Could not decode the QR: {e}");
                return ExitCode::FAILURE;
            }
        };

        println!("Detected {} QR(s)", results.len());

        for (index, data) in results.iter().enumerate() {
            println!("{}: {}", index + 1, data);
        }

    }

    return ExitCode::SUCCESS;
}
