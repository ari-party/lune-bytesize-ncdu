Rust project that outputs a [Ncdu JSON export file](https://dev.yorhel.nl/ncdu/jsonfmt) from a Roblox place or model file.
This allows you to analyze your place or model file to figure out what instances are the largest.

This calculates the asize and dsize. The asize is the byte size of an instance when it is reserialized. The dsize is the byte size consumed in the place or model file.
Roblox uses shared strings for deduplication, this is included in the dsize calculation. (This is why the total dsize may be larger than the input file's!)
