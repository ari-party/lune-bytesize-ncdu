A quick project I made that outputs a [Ncdu JSON export file](https://dev.yorhel.nl/ncdu/jsonfmt), which you can import to analyze the size of your Roblox placefile, or model.

This internally uses the [rbx_binary](https://docs.rs/rbx_binary/latest/rbx_binary/) crate from [rbx-dom](https://github.com/rojo-rbx/rbx-dom). There will likely be inaccuracies, as effectively every instance, including it's descendants, is saved into Roblox's binary format.

# How to use

Currently only supported on Linux.

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Ncdu](https://dev.yorhel.nl/ncdu)
- Set up the project: `./scripts/setup.sh`
- Run `./lune run script.luau INPUT_PLACE_OR_MODEL OUTPUT_JSON` (e.g. `./lune run script.luau input.rbxl output.json`)
- Run `ncdu -f OUTPUT_JSON` (e.g. `ncdu -f output.json`)
