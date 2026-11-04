```markdown
# 🪄 DropBG

**Drop it. It's gone. Paste it.**

DropBG is a free, open-source desktop application that automatically removes image backgrounds. Unlike other web-based services, **DropBG runs 100% offline**. Your images are never uploaded to any server, ensuring absolute privacy and security.

Simply drag & drop an image into the app, and the result is **automatically copied to your clipboard**. Just hit `Ctrl+V` in Photoshop, Figma, or any other app!

![DropBG App Screenshot](https://via.placeholder.com/800x400?text=Add+Your+DropBG+Screenshot+Here)
*(Replace the placeholder link above with your actual screenshot)*

## ✨ Features

- ⚡ **Super Fast**: The AI model is cached in memory (RAM), allowing background removal in just seconds.
- 🔒 **100% Offline & Private**: No internet connection required. No telemetry, no tracking, no data harvesting.
- 📋 **Auto-Copy to Clipboard**: Transparent images are instantly ready to paste without saving them to your disk.
- 🕒 **History**: Keeps track of your last 5 processed images. Click a thumbnail to copy it again.
- 💾 **Save to File**: Need a physical file? A save button is available right in the history section.
- 🎨 **Clean & Minimalist UI**: A "white-clean" design that keeps you focused on your work.

## 🚀 Download & Install

1. Go to the [Releases](https://github.com/gilangrahmandha212-lang/DropBG/releases) page.
2. Download the latest installer (`dropbg_0.1.0_x64-setup.exe`).
3. Run the installer and follow the on-screen instructions.
4. Open **DropBG** from your Windows Start Menu.

## 📖 How to Use

1. Open the DropBG application.
2. **Drag & drop** an image file (JPG/PNG/WebP) from File Explorer into the DropBG window.
3. Wait a few seconds until you see the green checkmark saying **"Successfully copied to clipboard!"**.
4. Open your target app (Photoshop, Figma, Word, etc.) and press `Ctrl + V`.
5. *Voila!* The background is magically gone.

## 🛠️ Tech Stack

- **[Tauri](https://tauri.app/)**: A lightweight desktop framework (a modern alternative to Electron).
- **[Rust](https://www.rust-lang.org/)**: A high-performance language used for the backend and image processing.
- **[React](https://react.dev/) + [Tailwind CSS](https://tailwindcss.com/)**: For a modern and responsive user interface.
- **[BiRefNet (ONNX)](https://github.com/ZhengPeng7/BiRefNet)**: A State-of-the-Art AI model for image segmentation, running on [ONNX Runtime](https://onnxruntime.ai/).

## 💻 Build from Source (For Developers)

If you want to contribute or build the app yourself:

```bash
# 1. Clone the repository
git clone https://github.com/gilangrahmandha212-lang/DropBG.git
cd DropBG

# 2. Install frontend dependencies
npm install

# 3. Download the AI Model
# You need to download the BiRefNet ONNX model and place it in:
# src-tauri/models/birefnet.onnx
# (Download it from the official BiRefNet releases)

# 4. Run in development mode
npm run tauri dev

# 5. Build for production (.exe / .msi)
npm run tauri build
```

## 📜 License & Credits

- This application's code is released under the **[MIT License](LICENSE)**.
- The **BiRefNet** AI model was created by ZhengPeng7 and is also released under the MIT License. Huge thanks to the researchers for their incredible work!
- Built with ❤️ to make life easier for editors and designers.

---
*If you find this app useful, please consider giving this repository a ⭐!*
