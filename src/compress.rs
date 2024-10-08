use humanize_duration::prelude::DurationExt;
use humanize_duration::Truncate;
use image::{ImageBuffer, Luma};
use lz4_flex::block::{compress_prepend_size, decompress_size_prepended};
use std::time::Instant;

fn main() {
    // Start the total time measurement
    let total_start = Instant::now();

    // Number of bits we want to generate: 10 billion bits, or 1.25 billion bytes.
    let num_bytes = 10_000_000_000 / 8;
    println!(
        "random bytes: {}",
        human_bytes::human_bytes(num_bytes as f64)
    );

    // Timing the allocation step
    let start = Instant::now();
    println!("Allocating...");
    let mut random_data = vec![0u8; num_bytes];
    println!("Took: {}\n", start.elapsed().human(Truncate::Nano));

    // Set some initial bits
    random_data[0] = 0b00000101;

    // Timing the random data generation step
    let start = Instant::now();
    println!("Filling vector with random data...");
    for i in 0..num_bytes / 1_000_000 {
        random_data[i] = rand::random();
    }
    for i in 0..num_bytes / 10 {
        let index = i * 2;
        random_data[index] = 0b11111111;
    }
    println!("Took: {}\n", start.elapsed().human(Truncate::Nano));

    // Timing the compression step
    let start = Instant::now();
    println!("Compressing data...");
    let compressed_data = compress_prepend_size(&random_data);
    println!(
        "Speed: {}/s",
        human_bytes::human_bytes(num_bytes as f64 / start.elapsed().as_secs_f64())
    );
    println!("Took: {}\n", start.elapsed().human(Truncate::Nano));

    // Timing the size calculation step
    let start = Instant::now();
    println!("Calculating sizes...");
    let original_size = random_data.len();
    let compressed_size = compressed_data.len();
    let compression_ratio = compressed_size as f64 / original_size as f64;
    println!("Took: {}\n", start.elapsed().human(Truncate::Nano));

    // Output the sizes and compression ratio
    println!(
        "Original size: {}",
        human_bytes::human_bytes(original_size as f64)
    );
    println!(
        "Compressed size: {}",
        human_bytes::human_bytes(compressed_size as f64)
    );
    println!("Compression ratio: {:.4}", compression_ratio);

    // Timing the decompression verification step
    let start = Instant::now();
    println!("\nDecompressing and verifying data..");
    let decompressed_data = decompress_size_prepended(&compressed_data).unwrap();
    assert_eq!(random_data, decompressed_data);
    println!(
        "Speed: {}/s",
        human_bytes::human_bytes(num_bytes as f64 / start.elapsed().as_secs_f64())
    );
    println!("Took: {}\n", start.elapsed().human(Truncate::Nano));

    // Timing the PNG output for uncompressed data
    println!("Outputting PNG images...");
    let start = Instant::now();
    output_bits_as_png(&random_data, "uncompressed.png");
    output_bits_as_png(&compressed_data, "compressed.png");
    println!("Took: {}\n", start.elapsed().human(Truncate::Nano));

    // Output total time
    println!(
        "Total execution time: {}",
        total_start.elapsed().human(Truncate::Nano)
    );
}

// Function to create a PNG image from the firstbytes
fn output_bits_as_png(data: &[u8], file_name: &str) {
    // let num_bytes_max = 20_000_000; // 20 million bytes -> 160 million bits
    let num_bytes_max = 125_000; // 1 million bits
    let num_bytes = std::cmp::min(data.len(), num_bytes_max);
    println!(
        "PNG image will contain {}",
        human_bytes::human_bytes(num_bytes as f64)
    );
    let data_to_display = &data[..num_bytes];
    let image_size = (num_bytes * 8) as u32;
    let dimension = ((image_size as f64).sqrt() as u32) + 1;
    println!("Creating image of size {}x{}", dimension, dimension);
    let mut img = ImageBuffer::new(dimension, dimension);

    for (i, byte) in data_to_display.iter().enumerate() {
        for bit_pos in 0..8 {
            let bit = (byte >> (7 - bit_pos)) & 1;
            let pixel_value: u8 = if bit == 1 { 255 } else { 0 };
            let x = (i * 8 + bit_pos) as u32 % dimension;
            let y = (i * 8 + bit_pos) as u32 / dimension;
            img.put_pixel(x, y, Luma([pixel_value]));
        }
    }

    img.save(file_name).unwrap();
    println!("Saved {} image", file_name);
}
