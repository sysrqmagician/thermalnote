# thermalnote

A terminal utility that lets you quickly write and print notes on a thermal printer.

## Overview

Thermalnote opens your preferred text editor, allows you to compose a note, and then sends it directly to a USB thermal printer. Perfect for quick reminders, to-do lists, recipes, or anything else you want to have as a physical note.

## Installation

### Prerequisites

- An ESC/POS-capable USB thermal printer
- Text editor of your choice

### Downloading

#### Pre-built from GitHub releases
Download the binary for your platform from [the latest release](https://github.com/sysrqmagician/thermalnote/releases/latest).

#### Building using Cargo

```bash
cargo install thermalnote
```

## Configuration

Thermalnote requires the following environment variables to be set:

| Variable | Description | Example |
|----------|-------------|---------|
| `THERMALNOTE_VENDOR` | Vendor ID of your thermal printer (hex without 0x prefix) | `0ed6` |
| `THERMALNOTE_PRODUCT` | Product ID of your thermal printer (hex without 0x prefix) | `06a6` |
| `THERMALNOTE_CODEPAGE` | Character encoding for text | `PC437` or `WPC1252` |
| `EDITOR` | Path to your preferred text editor | `/usr/bin/nano` or `/usr/bin/vim` |

You can find your printer's vendor and product IDs using:
- Linux: `lsusb`
- macOS: System Information utility
- Windows: Device Manager > USB devices > Properties

### Supported codepages

Run with the help flag set to see a list of supported code pages:
```bash
thermalnote -h
```

## Usage

1. Set up the required environment variables
2. Run thermalnote:
   ```bash
   thermalnote
   ```
3. Your chosen editor will open a temporary file
4. Write your note
5. Save and exit the editor
6. The note will be printed on your thermal printer and the temporary file deleted.

## Examples

For a bash/zsh setup, you might add this to your shell configuration:

```bash
export THERMALNOTE_VENDOR="0ed6"
export THERMALNOTE_PRODUCT="06a6"
export THERMALNOTE_CODEPAGE="PC437"
export EDITOR="/usr/bin/vim"
```

## Troubleshooting

- **"Error opening USB device"**: Make sure your vendor and product IDs are correct and the printer is connected
- **Editor doesn't open**: Ensure `EDITOR` is set to a valid path without arguments
- **Text doesn't print correctly**: Try changing the codepage to match your language/character needs
- **Permission denied**: You might need to add udev rules for USB access on Linux

  To allow all users on your system to write to your printer, create the following file with your vendor and product id:
  ```
  /etc/udev/rules.d/999-thermalnote.rules

  SUBSYSTEM=="usb", ATTRS{idVendor}=="0ed6", ATTRS{idProduct}=="06a6", MODE="0666"
  ```

## License

MIT. See [LICENSE](LICENSE) file.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
