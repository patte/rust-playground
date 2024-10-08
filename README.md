# rust playground

Testing out rust features and libraries.

## Run
```bash
cargo run --bin [compress|stream_nokhwa|video_signal]
```

## Examples

### lz4 compression
```bash
cargo run --bin compress --release

random bytes: 1.3 GB
Allocating...
Took: 82µs 625ns

Filling vector with random data...
Took: 63ms 149µs 42ns

Compressing data...
Speed: 9.8 GB/s
Took: 126ms 928µs 250ns

Calculating sizes...
Took: 792ns

Original size: 1.3 GB
Compressed size: 4.9 MB
Compression ratio: 0.0039

Decompressing and verifying data..
Speed: 4.6 GB/s
Took: 272ms 231µs 166ns

Outputting PNG images...
PNG image will contain 125 KB
Creating image of size 1001x1001
Saved uncompressed.png image
PNG image will contain 125 KB
Creating image of size 1001x1001
Saved compressed.png image
Took: 7ms 461µs 292ns

Total execution time: 470ms 129µs 791ns
```
Machine: MacBook Air 2023 M2 24GB