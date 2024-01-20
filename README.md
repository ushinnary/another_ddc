# 🌞 Screen Brightness Management

## Description
Let you manage your screen's brightness with a simple command line. 💡

## Disclaimer

⚠️ Please be aware that this tool directly interacts with your system's screen brightness settings. While it has been tested and is intended to be safe, there is always the potential for unexpected behavior or system instability, especially on different hardware or software configurations. 

Use this tool at your own risk. The author or contributors cannot be held responsible for any damage or issues that may occur from using this tool. 🧑‍💻

## Installation
### Windows
```bash
cargo run --release
.\add-to-path.bat
```

## Usage
Brightness will affect all screens. 🖥️

```bash
ddc_brightness.exe -b 50    # Set brightness to 50%
ddc_brightness.exe -b 50 -s # Set brightness to 50% smoothly
ddc_brightness.exe -i       # Increase brightness by 1%
ddc_brightness.exe -d       # Decrase brightness by 1%
```