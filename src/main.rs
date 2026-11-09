use std::process::ExitCode;
use clap::Parser;
use qr_gen::qr::{encode_qr, decode_qr};
use qr_gen::clipboard::{
    copy_image_to_clipboard,
    retrieve_image_from_clipboard
};

#[derive(Parser, Debug)]
#[command(version, about = "Generate QR code quickly")]
struct Cli {
    /// Switch to scan mode that scan the image in clipboard for QR code
    #[arg(short, long)]
    scan: bool,

    /// The data to convert to QR code
    #[arg(required_unless_present = "scan")]
    data: Vec<String>
}

fn main() -> ExitCode {
    let args = Cli::parse();

    if !args.scan {
        let data = args.data.join(" ");
        let png = match encode_qr(&data) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                return ExitCode::FAILURE;
            }
        };

        if let Err(e) = png.save("code.png") {
            eprintln!("ERROR: {}", e);
            return ExitCode::FAILURE;
        }

        if let Err(e) = copy_image_to_clipboard(png) {
            eprintln!("ERROR: {}", e);
            return ExitCode::FAILURE;
        }

        println!("QR code copied to clipboard and saved to code.png!");

    } else {
        let img = match retrieve_image_from_clipboard() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                return ExitCode::FAILURE;
            }
        };

        let results = match decode_qr(img) {
            Ok(res) => res,
            Err(e) => {
                eprintln!("ERROR: {}", e);
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
