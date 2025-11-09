# 🔤 Font Setup Guide - GUI Chinese Display Fix

## 📋 Problem Description

GUI applications may display Chinese characters as garbled text (showing as squares) because the default egui font does not support Chinese characters.

## 🎯 Solutions

### Method 1: Automatic System Font Detection (Recommended)

The program has built-in automatic font detection and will try the following paths:

#### Windows System
- `C:\Windows\Fonts\msyh.ttc` (Microsoft YaHei)
- `C:\Windows\Fonts\simhei.ttf` (SimHei)
- `C:\Windows\Fonts\simsun.ttc` (SimSun)

#### macOS System
- `/System/Library/Fonts/PingFang.ttc`
- `/System/Library/Fonts/Hiragino Sans GB.ttc`

#### Linux System
- `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`
- `/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc`
- `/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc`

### Method 2: Manual Font File Placement

If automatic detection fails, you can manually download font files:

1. **Download fonts**:
   - [Source Han Sans](https://github.com/adobe-fonts/source-han-sans/releases)
   - [Microsoft YaHei](https://www.microsoft.com/en-us/download/details.aspx?id=26120)
   - [Noto Sans CJK](https://fonts.google.com/noto/specimen/Noto+Sans+SC)

2. **Place fonts**:
   Rename the downloaded font file to any of the following names and place it in the project root directory:
   - `msyh.ttc`
   - `simhei.ttf`
   - `source-han-sans.otf`

### Method 3: Install Chinese Fonts Using System Package Manager

#### Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install fonts-noto-cjk
sudo apt-get install fonts-wqy-microhei
```

#### Fedora/CentOS:
```bash
sudo dnf install google-noto-cjk-fonts
sudo dnf install wqy-microhei-fonts
```

#### Arch Linux:
```bash
sudo pacman -S noto-fonts
sudo pacman -S adobe-source-han-sans-cn-fonts
```

## 🚀 Usage Steps

1. **Ensure fonts are available**:
   ```bash
   # Check if Windows font exists
   ls C:\Windows\Fonts\msyh.ttc
   
   # Check if macOS font exists
   ls /System/Library/Fonts/PingFang.ttc
   
   # Check if Linux font exists
   ls /usr/share/fonts/truetype/wqy/wqy-zenhei.ttc
   ```

2. **Run the GUI program**:
   ```bash
   cargo run --bin prime-gui
   ```

3. **Check console output**:
   The program will display font loading information on startup:
   ```
   Successfully loaded font: C:\Windows\Fonts\msyh.ttc
   ```
   Or if no font is found:
   ```
   Warning: No suitable font found for Chinese characters, using default font
   ```

## 🔍 Troubleshooting

### Problem 1: Chinese characters still display as squares

**Solution**:
- Ensure the `.exe` file's read-only attribute is cleared
- Try other font files
- Check if the font file is corrupted

### Problem 2: Font loading failed

**Solution**:
- Run the program with administrator privileges
- Confirm the font file path is correct
- Try placing the font file in the project root directory

### Problem 3: GUI cannot start

**Solution**:
- Check if antivirus software is blocking it
- Ensure all Rust dependencies are correctly installed
- Try running `cargo build --release`

## 📊 Supported Font Formats

- ✅ `.ttf` (TrueType Font)
- ✅ `.ttc` (TrueType Collection)
- ✅ `.otf` (OpenType Font)
- ❌ `.woff` (Web Open Font Format)
- ❌ `.eot` (Embedded OpenType)

## 🎨 Recommended Fonts

### Best Experience
1. **Microsoft YaHei** (msyh.ttc) - Built-in on Windows
2. **Source Han Sans** (source-han-sans.otf) - Adobe open source font
3. **Noto Sans CJK** - Google open source font

### Alternative Options
1. **SimHei** (simhei.ttf) - Built-in on Windows
2. **SimSun** (simsun.ttc) - Built-in on Windows
3. **WenQuanYi Micro Hei** (wqy-microhei.ttc) - Common on Linux

## 💡 Tips

1. **Font size**: The program automatically adjusts font size to fit the interface
2. **Performance impact**: Font loading has negligible impact on program startup time
3. **Cross-platform**: The same font file works normally on Windows, macOS, and Linux
4. **Memory usage**: Font files are typically only a few MB, memory usage is negligible

## 🆘 Getting Help

If you still encounter font display issues:

1. Check the font loading information in the console output
2. Confirm that your operating system supports displaying Chinese
3. Try using different font files
4. Check your system's locale settings

---

**Note**: If all methods fail, the GUI can still run normally, just that Chinese characters will display as squares. The core functionality of the program (prime number calculation) is not affected.