# Leptos_Tailwind_Tauri_Template
Build cross-platform applications using Leptos/Tailwind/Tauri.
> 使用 Leptos+Tailwind+Tauri 构建跨平台应用。

TIPS:
- This project is only a template and does not contain any actual functions.
- Some areas may encounter download failures and need to be resolved properly.
> 提示:
> - 本项目仅为模板，不包含任何实际功能。
> - 某些区域可能遇到下载失败的情况，需要妥善解决。

## I. Project Initialization
> I. 项目初始化
### 1. Environment Preparation
> 1. 环境准备
#### Check cargo
> 检查 cargo
~~~
cargo -V
~~~
#### Check npm
> 检查 npm
~~~
npm -v
~~~

### 2. Environment Initialization
> 2. 初始化环境
#### Clone the repository
> 克隆仓库
~~~
git clone https://github.com/heming-fit/leptos_tailwind_tauri_template.git
~~~
#### Install dependencies
> 安装依赖
~~~
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli tauri-cli
npm install
~~~

## II. Project release
> II. 项目发布
~~~
cargo tauri build --release
~~~

## III. Project Development
> III. 项目开发
~~~
cargo tauri dev --release
~~~