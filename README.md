# 🎓 Deysis TUI

> Fast, privacy-focused terminal client for location-based attendance systems

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0-green.svg)](https://github.com/yourusername/deysis-tui/releases)

## ✨ Features

- 🔐 **Multi-account management** - Switch between saved accounts
- 📱 **6-digit code entry** - Large ASCII art display
- 🌍 **Location faking** - Auto-generated coordinates (±10-15m of the faculty)
- 🎨 **Modern TUI** - Colorful, keyboard-driven interface
- ⚡ **Fast & async** - Non-blocking operations

---

## 🚀 Installation

```bash
git clone https://github.com/umceko/deysis-tui.git
cd deysis-tui
cargo run --release
```

**Requirements**: Rust 1.70+

---

## 📖 How It Works

1. **Login** - Credentials saved automatically
2. **Home Screen** - Select "Kod Gir"
3. **Enter Code** - Type 6-digit attendance code
4. **Auto-submit** - Location randomized and sent
5. **Confirmation** - Instant success/failure feedback

**Privacy Note**: Your location is randomized within a 10-15 meter radius of the faculty. The system shows school coordinates while ensuring you're actually present on premises.

---

## ⌨️ Controls

| Key         | Action        |
|-------------|---------------|
| `↑` `↓`     | Navigate      |
| `Enter`     | Confirm       |
| `Esc`       | Back          |
| `Backspace` | Delete        |
| `d`         | Delete account |

---

## 📸 Interface

![Screenshot](https://i.imgur.com/jUjm5Nq.png)
![Screenshot](https://i.imgur.com/FwigLBc.png)

---

## 🔧 Development

```bash
# Run in dev mode
cargo run

# Build release
cargo build --release
```

---

## 🤝 Contributing

1. Fork the repo
2. Create feature branch
3. Commit changes
4. Push and open PR

---

## 📝 License

MIT License - See LICENSE file

---

## 📮 Contact

**Author**: UMCEKO  
**Email**: umutcevdetkocak@gmail.com  
**GitHub**: [@UMCEKO](https://github.com/UMCEKO)

---

## ⭐ Support

Star this repo if it helped you!

---

**Built with Rust** 🦀