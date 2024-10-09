/// code to read webcam, apply motion visualization, stream to mjpeg over http
/// requires opencv to be installed. Follow:
/// https://github.com/twistedfall/opencv-rust/blob/master/INSTALL.md
/// Source: https://github.com/twistedfall/opencv-rust/blob/master/examples/video_capture_http_stream.rs
use std::io::{Cursor, Write};
use std::net::{SocketAddr, TcpListener};

use image::codecs::jpeg::JpegEncoder;
use image::{imageops, ImageBuffer, Rgb, Rgba, RgbaImage};
// VideoCaptureTrait doesn't get used when binding to opencv 3.4
#[allow(unused_imports)]
use opencv::videoio::VideoCaptureTrait;

use opencv::core::{Mat, Vector, CV_8UC3};
use opencv::imgcodecs::{imencode, IMWRITE_JPEG_QUALITY};
use opencv::videoio::{VideoCapture, VideoCaptureTraitConst, CAP_ANY};
use opencv::{prelude::*, Result};

use image::imageops::*;
use mat2image::{bgr_buf_to_rgba_image, ToImage};

const BASE_RESPONSE: &[u8] =
    b"HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // select image backend
    let mut image_backend = "image";
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "opencv" || args[1] == "image") {
        image_backend = &args[1];
    }
    println!("Using image backend: {}", image_backend);

    // Open the default camera
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    assert!(cam.is_opened()?, "Unable to open default camera!");

    // Bind listener to a port
    let address: SocketAddr = "127.0.0.1:8080".parse()?;
    let listener = TcpListener::bind(address)?;
    println!("Listening for connections at {}", address.to_string());

    // Accept the first incoming connection
    let (mut stream, addr) = listener.accept()?;
    println!("Client connected: {}", addr);

    // Write intial response
    stream.write_all(BASE_RESPONSE)?;

    let mut buffer = Mat::default(); // type: "CV_8UC3"
    match image_backend {
        "opencv" => {
            let encode_params = Vector::from_slice(&[IMWRITE_JPEG_QUALITY, 80]);
            let mut formatted_bytes = Vector::default();
            let image_format = "jpeg";
            let mut prev_image: Option<Mat> = None;
            loop {
                cam.read(&mut buffer)?;
                if buffer.size()?.width <= 0 {
                    continue;
                }

                let mut current_frame = buffer.clone();

                // Invert previous frame and blend on the current frame
                if let Some(mut prev) = prev_image.take() {
                    // invert colors
                    opencv::core::bitwise_not(&prev.clone(), &mut prev, &Mat::default())?;

                    // blend images
                    opencv::core::add_weighted(
                        &prev,
                        0.5,
                        &current_frame.clone(),
                        0.5,
                        0.0,
                        &mut current_frame,
                        CV_8UC3,
                    )?;
                }

                prev_image = Some(buffer.clone());

                //println!("{:?}", buffer);
                imencode(
                    format!(".{}", image_format).as_str(),
                    &current_frame,
                    &mut formatted_bytes,
                    &encode_params,
                )?;

                let packet = {
                    let header = format!(
                        "--frame\r\nContent-Type: image/{}\r\nContent-Length: {}\r\n\r\n",
                        image_format,
                        formatted_bytes.len()
                    );
                    [header.as_bytes(), formatted_bytes.as_slice()].concat()
                };
                stream.write_all(&packet)?;
            }
        }
        "image" => {
            let mut prev_image: Option<RgbaImage> = None;
            let image_format = "jpeg";
            loop {
                cam.read(&mut buffer)?;
                if buffer.size()?.width <= 0 {
                    continue;
                }

                let imb = buffer.as_image_buffer()?;
                let rgba_image = bgr_buf_to_rgba_image(imb);
                let mut out_img = rgba_image.clone();

                // Invert previous frame and blend on the current frame
                if let Some(mut prev) = prev_image.take() {
                    // invert colors
                    colorops::invert(&mut prev);

                    // set alpha channel to 125
                    for pixel in prev.enumerate_pixels_mut() {
                        let Rgba([r, g, b, _a]) = *pixel.2;
                        *pixel.2 = Rgba([r, g, b, 125]);
                    }

                    // blend images
                    imageops::overlay(&mut out_img, &prev, 0, 0);
                }

                prev_image = Some(rgba_image);

                //let out_img_rgb = image::DynamicImage::ImageRgba8(out_img).to_rgb8();
                let out_img_rgb = rgba8_to_rgb8(out_img);
                let jpeg_bytes = rgb_image_to_jpeg_bytes(out_img_rgb, 80);

                let packet = {
                    let header = format!(
                        "--frame\r\nContent-Type: image/{}\r\nContent-Length: {}\r\n\r\n",
                        image_format,
                        jpeg_bytes.len()
                    );
                    [header.as_bytes(), jpeg_bytes.as_slice()].concat()
                };
                stream.write_all(&packet)?;
            }
        }
        _ => {
            panic!("Invalid variant");
        }
    }
}

fn rgb_image_to_jpeg_bytes(image: ImageBuffer<Rgb<u8>, Vec<u8>>, quality: u8) -> Vec<u8> {
    let mut jpeg_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut jpeg_bytes);
    let mut encoder = JpegEncoder::new_with_quality(&mut cursor, quality);
    encoder
        .encode(
            &image,
            image.width(),
            image.height(),
            image::ExtendedColorType::Rgb8,
        )
        .unwrap();
    jpeg_bytes
}

// Convert RGBA8 image to RGB8 image
// alternative: let rgb = image::DynamicImage::ImageRgba8(out_img).to_rgb8();
// https://play.rust-lang.org/?version=stable&mode=release&edition=2018&gist=b5b7977e168b13b8377d462c8c9c8d34
fn rgba8_to_rgb8(
    input: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = input.width() as usize;
    let height = input.height() as usize;

    // Get the raw image data as a vector
    let input: &Vec<u8> = input.as_raw();

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width * height * 3];

    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for (output, chunk) in output_data.chunks_exact_mut(3).zip(input.chunks_exact(4)) {
        // ... and copy each of them to output, leaving out the A byte
        output.copy_from_slice(&chunk[0..3]);
    }

    // Construct a new image
    image::ImageBuffer::from_raw(width as u32, height as u32, output_data).unwrap()
}
