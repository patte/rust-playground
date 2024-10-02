use bit_vec::BitVec;
use image::{ImageBuffer, Luma};
use std::fs;
use std::process::Command;

pub fn write_video(data: &BitVec, fps: u32, width: u32, height: u32) {
    let total_frames = data.len();
    let duration_seconds = (total_frames as f64 / fps as f64).ceil();
    println!(
        "Total frames: {} duration: {}",
        total_frames, duration_seconds
    );

    // Create a directory for the frames
    let frames_dir = "frames";
    fs::remove_dir_all(frames_dir).ok();
    fs::create_dir_all(frames_dir).expect("Failed to create output directory");

    // Generate frames and save them as images
    for (i, value) in data.iter().enumerate() {
        // Create a new grayscale image (Luma)
        let img = ImageBuffer::from_fn(width, height, |_x, _y| {
            if value {
                Luma([255u8]) // White pixel
            } else {
                Luma([0u8]) // Black pixel
            }
        });

        // Save the image as a PNG file
        let frame_path = format!("{}/frame_{:04}.png", frames_dir, i);
        img.save(frame_path).expect("Failed to save frame");
    }

    // Use ffmpeg to combine the images into a video
    let output_video = "output.mp4";
    let ffmpeg_status = Command::new("ffmpeg")
        .args(&[
            "-framerate",
            &fps.to_string(),
            "-i",
            &format!("{}/frame_%04d.png", frames_dir),
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            //"-t",
            //&duration_seconds.to_string(),
            "-y",
            output_video,
        ])
        .status()
        .expect("Failed to run ffmpeg");

    if ffmpeg_status.success() {
        println!("Video saved as {}", output_video);
    } else {
        eprintln!("Error: Failed to generate video");
    }
}

//use ffmpeg_next::software::scaling;
use ffmpeg_next::{codec, format, frame, media};

pub fn read_video() -> BitVec {
    let video_file = "output.mp4";
    let mut data = BitVec::new();

    ffmpeg_next::init().unwrap();

    let mut ictx = format::input(&video_file).expect("Failed to open input file");

    let input_stream = ictx
        .streams()
        .best(media::Type::Video)
        .expect("Failed to find video stream");
    let video_stream_index = input_stream.index();

    let codec =
        codec::Context::from_parameters(input_stream.parameters()).expect("Failed to open codec");

    let mut video_decoder = codec
        .decoder()
        .video()
        .expect("Failed to get video decoder");

    //let mut scaler = scaling::Context::get(
    //    video_decoder.format(),
    //    video_decoder.width(),
    //    video_decoder.height(),
    //    format::Pixel::YUV420P,
    //    video_decoder.width(),
    //    video_decoder.height(),
    //    scaling::Flags::BILINEAR,
    //)
    //.expect("Failed to create scaler");

    let mut frame = frame::Video::empty();

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            video_decoder
                .send_packet(&packet)
                .expect("Failed to send packet to decoder");

            while video_decoder.receive_frame(&mut frame).is_ok() {
                data.push(get_bit_from_frame(&frame));

                //let mut grayscale_frame = frame::Video::empty();
                //scaler
                //    .run(&frame, &mut grayscale_frame)
                //    .expect("Failed to scale frame");
                //
                //////println!("frame format: {:?}", frame.format());
                //println!("frame data: {:?}", grayscale_frame.data(0)[0]);
            }
        }
    }

    // drain
    video_decoder
        .send_eof()
        .expect("Failed to send EOF to decoder");
    while video_decoder.receive_frame(&mut frame).is_ok() {
        data.push(get_bit_from_frame(&frame));
    }

    data
}

fn get_bit_from_frame(frame: &frame::Video) -> bool {
    frame.data(0)[0] > 128
}
