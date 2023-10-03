# generate-app-icons

[![Crates.io](https://img.shields.io/crates/v/generate-app-icons.svg)](https://crates.io/crates/generate-app-icons)
[![Rust](https://img.shields.io/badge/rust-1.56.1%2B-blue.svg?maxAge=3600)](https://gitlab.com/andrew_ryan/generate-app-icons)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://gitlab.com/andrew_ryan/generate-app-icons/-/raw/master/LICENSE)

## install
```sh
cargo install generate-app-icons
```
## Usage
```sh
generate-app-icons <icon_path>
```
### Example
```sh
generate-app-icons ./assets/icon.png

tree ./app_icons

app_icons
├── Assets.xcassets
│   └── AppIcon.appiconset
│       ├── 100.png
│       ├── 1024.png
│       ├── 114.png
│       ├── 120.png
│       ├── 128.png
│       ├── 144.png
│       ├── 152.png
│       ├── 16.png
│       ├── 167.png
│       ├── 172.png
│       ├── 180.png
│       ├── 196.png
│       ├── 20.png
│       ├── 216.png
│       ├── 256.png
│       ├── 29.png
│       ├── 32.png
│       ├── 40.png
│       ├── 48.png
│       ├── 50.png
│       ├── 512.png
│       ├── 55.png
│       ├── 57.png
│       ├── 58.png
│       ├── 60.png
│       ├── 64.png
│       ├── 72.png
│       ├── 76.png
│       ├── 80.png
│       ├── 87.png
│       ├── 88.png
│       └── Contents.json
├── android
│   ├── mipmap-hdpi
│   │   └── ic_launcher.png
│   ├── mipmap-mdpi
│   │   └── ic_launcher.png
│   ├── mipmap-xhdpi
│   │   └── ic_launcher.png
│   ├── mipmap-xxhdpi
│   │   └── ic_launcher.png
│   └── mipmap-xxxhdpi
│       └── ic_launcher.png
├── appstore.png
└── playstore.png
```