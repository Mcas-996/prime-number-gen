# 🔤 字体设置指南 - GUI中文显示修复

## 📋 问题描述

GUI应用程序在显示中文时可能出现乱码（显示为方框），这是因为egui默认字体不支持中文字符。

## 🎯 解决方案

### 方法一：自动系统字体检测（推荐）

程序已经内置了自动字体检测功能，会尝试以下路径：

#### Windows 系统
- `C:\Windows\Fonts\msyh.ttc` (微软雅黑)
- `C:\Windows\Fonts\simhei.ttf` (黑体)
- `C:\Windows\Fonts\simsun.ttc` (宋体)

#### macOS 系统
- `/System/Library/Fonts/PingFang.ttc`
- `/System/Library/Fonts/Hiragino Sans GB.ttc`

#### Linux 系统
- `/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf`
- `/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc`
- `/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc`

### 方法二：手动放置字体文件

如果自动检测失败，您可以手动下载字体文件：

1. **下载字体**：
   - [思源黑体 (Source Han Sans)](https://github.com/adobe-fonts/source-han-sans/releases)
   - [微软雅黑](https://www.microsoft.com/zh-cn/download/details.aspx?id=26120)
   - [Noto Sans CJK](https://fonts.google.com/noto/specimen/Noto+Sans+SC)

2. **放置字体**：
   将下载的字体文件重命名为以下任一名称，并放置在项目根目录：
   - `msyh.ttc`
   - `simhei.ttf`
   - `source-han-sans.otf`

### 方法三：使用系统包管理器安装中文字体

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

## 🚀 使用步骤

1. **确保字体可用**：
   ```bash
   # 检查Windows字体是否存在
   ls C:\Windows\Fonts\msyh.ttc
   
   # 检查macOS字体是否存在
   ls /System/Library/Fonts/PingFang.ttc
   
   # 检查Linux字体是否存在
   ls /usr/share/fonts/truetype/wqy/wqy-zenhei.ttc
   ```

2. **运行GUI程序**：
   ```bash
   cargo run --bin prime-gui
   ```

3. **检查控制台输出**：
   程序启动时会显示字体加载信息：
   ```
   成功加载字体: C:\Windows\Fonts\msyh.ttc
   ```
   或者如果找不到字体：
   ```
   警告: 未找到适合中文字符的字体，使用默认字体
   ```

## 🔍 故障排除

### 问题1：中文仍然显示为方框

**解决方案**：
- 确认清除了`.exe`文件的只读属性
- 尝试其他字体文件
- 检查字体文件是否损坏

### 问题2：字体加载失败

**解决方案**：
- 以管理员权限运行程序
- 确认字体文件路径正确
- 尝试将字体文件放在项目根目录

### 问题3：GUI无法启动

**解决方案**：
- 检查是否有防病毒软件阻止
- 确认所有Rust依赖正确安装
- 尝试运行`cargo build --release`

## 📊 支持的字体格式

- ✅ `.ttf` (TrueType Font)
- ✅ `.ttc` (TrueType Collection)
- ✅ `.otf` (OpenType Font)
- ❌ `.woff` (Web Open Font Format)
- ❌ `.eot` (Embedded OpenType)

## 🎨 推荐字体

### 最佳体验
1. **微软雅黑** (msyh.ttc) - Windows系统自带
2. **思源黑体** (source-han-sans.otf) - Adobe开源字体
3. **Noto Sans CJK** - Google开源字体

### 备选方案
1. **黑体** (simhei.ttf) - Windows系统自带
2. **宋体** (simsun.ttc) - Windows系统自带
3. **文泉驿微米黑** (wqy-microhei.ttc) - Linux常用

## 💡 小贴士

1. **字体大小**：程序会自动调整字体大小以适应界面
2. **性能影响**：字体加载对程序启动时间影响微乎其微
3. **跨平台**：同一字体文件在Windows、macOS、Linux上都能正常工作
4. **内存占用**：字体文件通常只有几MB，内存占用可忽略不计

## 🆘 获取帮助

如果仍然遇到字体显示问题：

1. 查看控制台输出的字体加载信息
2. 确认操作系统支持显示中文
3. 尝试使用不同的字体文件
4. 检查系统的locale设置

---

**注意**：如果所有方法都失败，GUI仍然可以正常运行，只是中文会显示为方框。程序的核心功能（素数计算）不受影响。