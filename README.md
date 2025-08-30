Rust project that outputs a [ncdu JSON file](https://dev.yorhel.nl/ncdu/jsonfmt) from a Roblox place or model file.
This allows you to analyze your place or model file to figure out what instances are the largest.

This calculates the asize and dsize. The asize is the byte size of an instance when it is reserialized. The dsize is the byte size consumed in the place or model file.
Roblox uses shared strings for deduplication, this is included in the dsize calculation. (This is why the total dsize may be larger than the input file's!)

**Requirements**:

- [Rust](https://www.rust-lang.org/tools/install)
- Analyzer compatible with the ncdu JSON format, e.g. [ncdu](https://dev.yorhel.nl/ncdu) or [gdu](https://github.com/dundee/gdu)

## Setup repo

```bash
git clone --recurse-submodules https://github.com/ari-party/rbx-bytesize-ncdu.git
# or
git clone https://github.com/ari-party/rbx-bytesize-ncdu.git
cd rbx-bytesize-ncdu
git submodule update --init --recursive
```

## Run locally

```bash
cargo run -- input.rbxl output.json
# or optimized release build
cargo run -r -- input.rbxl output.json

ncdu -f output.json
# or
gdu -f output.json
```

## Building and installing

Installs the built executable globally, accessible as `rbx-bytesize-ncdu`

```bash
cargo build
# or optimized release build
cargo build -r

cargo install --path .

rbx-bytesize-ncdu input.rbxl output.json

ncdu -f output.json
# or
gdu -f output.json
```
