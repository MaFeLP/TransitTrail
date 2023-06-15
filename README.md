# Tauri + Svelte + Typescript

This template should help get you started developing with Tauri, Svelte and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Releasing
1. Edit [package.json](./package.json), [Cargo.toml](src-tauri/Cargo.toml), [tauri.conf.json](src-tauri/tauri.conf.json) with the new version number
2. Commit and create a tag (preferably signed).
3. Push to main (also include tags!)
4. Run the workflow [`Publish Release Assets`](https://github.com/MaFeLP/TransitTrail/actions/workflows/upload-release.yml)
5. Edit the draft [release](https://github.com/MaFeLP/TransitTrail/releases)
6. Publish it!
7. Run the workflow [`Build Documentation PDFs`](https://github.com/MaFeLP/TransitTrail/actions/workflows/documentation.yml) with the version number as an input (version number should be the same as the tag name).
