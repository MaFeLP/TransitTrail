# Tauri + Svelte + Typescript

This template should help get you started developing with Tauri, Svelte and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Releasing
1. Edit [package.json](./package.json), [Cargo.toml](src-tauri/Cargo.toml), [tauri.conf.json](src-tauri/tauri.conf.json) with the new version number
2. Push to main
3. Run the workflow [`Publish Release Assets`](https://github.com/MaFeLP/TransitTrail/actions/workflows/upload-release.yml)
4. Edit the new [release](https://github.com/MaFeLP/TransitTrail/releases)
5. Publish it!
