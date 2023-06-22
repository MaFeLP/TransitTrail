[![Build and test the Application](https://github.com/MaFeLP/TransitTrail/actions/workflows/tauri-app.yml/badge.svg)](https://github.com/MaFeLP/TransitTrail/actions/workflows/tauri-app.yml) [![Build documentation PDFs](https://github.com/MaFeLP/TransitTrail/actions/workflows/documentation.yml/badge.svg)](https://github.com/MaFeLP/TransitTrail/actions/workflows/documentation.yml) [![Publish Release Assets](https://github.com/MaFeLP/TransitTrail/actions/workflows/upload-release.yml/badge.svg)](https://github.com/MaFeLP/TransitTrail/actions/workflows/upload-release.yml) [![Winnipeg API Client Test](https://github.com/MaFeLP/TransitTrail/actions/workflows/transit-api-client.yml/badge.svg)](https://github.com/MaFeLP/TransitTrail/actions/workflows/transit-api-client.yml)

# TransitTrail

> **NOTE**: This project was developed for a ComputerScience Course and is not ready for production, yet!

## Installation
1. Head over to the [releases page](https://github.com/MaFeLP/TransitTrail/releases)
2. Download the latest release for your platform. We currently only support x86\_64!

## Development
### Building from Source
#### Prerequisities
- [cargo](https://www.rust-lang.org/tools/install)
- [tauri](https://tauri.app/v1/guides/getting-started/prerequisites)
- [pnpm](https://pnpm.io/installation)

#### Building
```bash
git clone --recurse ssh://git@github.com/MaFeLP/TransitTrail.git
cd TransitTrail
pnpm install
cargo tauri build
```

### Recommended IDE Setup
[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Releasing
1. Edit [package.json](./package.json), [Cargo.toml](src-tauri/Cargo.toml), [tauri.conf.json](src-tauri/tauri.conf.json) with the new version number
2. Commit and create a tag (preferably signed).
3. Push to main (also include tags!)
4. Run the workflow [`Publish Release Assets`](https://github.com/MaFeLP/TransitTrail/actions/workflows/upload-release.yml)
5. Edit the draft [release](https://github.com/MaFeLP/TransitTrail/releases)
6. Publish it!
7. Run the workflow [`Build Documentation PDFs`](https://github.com/MaFeLP/TransitTrail/actions/workflows/documentation.yml) with the version number as an input (version number should be the same as the tag name).

## License
TransitTrail - Navigate Winnipeg Transit with a different style\
Copyright (C) 2023 - Foxx Azalea Pinkerton, Max Fehlinger

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT
ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see <https://www.gnu.org/licenses/>.

