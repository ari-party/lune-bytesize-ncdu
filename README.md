A quick project I made that outputs a [Ncdu JSON export file](https://dev.yorhel.nl/ncdu/jsonfmt), which you can import to analyze the size of your Roblox placefile, or model.

# How to use

Currently only supported on Linux.

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Ncdu](https://dev.yorhel.nl/ncdu)
- Set up the project: `./scripts/setup.sh`
- Run `./lune run script.luau INPUT_PLACE_OR_MODEL OUTPUT_JSON` (e.g. `./lune run script.luau input.rbxl output.json`)
- Run `ncdu -f OUTPUT_JSON` (e.g. `ncdu -f output.json`)
