# rust playground

Testing out rust features and libraries.

## Run
```bash
cargo run --bin [compress|stream_nokhwa|video_signal]
```

## Examples

### lz4 compression
```bash
cargo run --bin compress
random bytes: 1.3 GB
Allocating...
Took: 114µs 875ns

Filling vector with random data...
Took: 1s 497ms 637µs 750ns

Compressing data...
Speed: 187.8 MB/s
Took: 6s 657ms 178µs 250ns

Calculating sizes...
Took: 834ns

Original size: 1.3 GB
Compressed size: 4.9 MB
Compression ratio: 0.0039

Decompressing and verifying data..
Speed: 216 MB/s
Took: 5s 786ms 288µs 916ns

Outputting PNG images...
PNG image will contain 125 KB
Creating image of size 1001x1001
Saved uncompressed.png image
PNG image will contain 125 KB
Creating image of size 1001x1001
Saved compressed.png image
Took: 352ms 414µs 625ns

Total execution time: 14s 294ms 296µs 708ns
```
Machine: MacBook Air 2023 M2 24GB