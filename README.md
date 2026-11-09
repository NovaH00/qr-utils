# qr-utils

QR tools to use in the CLI.

# Installation

## From cargo
```shell
cargo install qr-utils
```

## From source
```
git clone https://github.com/NovaH00/qr-utils.git
cd qr-utils
cargo build --release
```

# Usage

To generate a QR code with the content "Hello, World!", and push the generate QR code to the clipboard.
```shell
qr-utils gen Hello, World!
```

To generate a QR code with the content "Goodbye!", push the generated QR code to the clipboard, AND save
it to a given directory.
```shell
qr-utils gen Goodbye! -s /path/to/directory
```

To scan for QR codes with the image currently in the clipboard.
```shell
qr-utils scan
```

To scan for QR codes with the image currently in the file system.
```shell
qr-utils scan -i /path/to/image.png
```
