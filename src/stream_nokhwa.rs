use std::{
    fs,
    sync::{Arc, Mutex},
    time::Duration,
};

//use image::{ImageBuffer, Rgb};
use nokhwa::{
    native_api_backend,
    pixel_format::RgbFormat,
    query,
    utils::{
        CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType, Resolution,
    },
    CallbackCamera,
};

fn main() {
    nokhwa::nokhwa_initialize(|x| {
        println!("Nokhwa Initalized: {x}");
        nokhwa_main()
    });
    std::thread::sleep(Duration::from_millis(15000));
}

fn nokhwa_main() {
    let backend = native_api_backend().unwrap();
    let devices = query(backend).unwrap();
    println!("There are {} available cameras.", devices.len());
    for device in devices {
        println!("{device}");
    }

    let index = CameraIndex::Index(0);
    let resolution = Resolution::new(1280, 720);
    let fourcc = FrameFormat::NV12;
    let fps = 30;
    let camera_format = CameraFormat::new(resolution, fourcc, fps);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(camera_format));
    println!("Requested Format: {:?}", requested);

    //let mut previousImage: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(0, 0);
    let frames = Arc::new(Mutex::new(Vec::new()));
    let frames_clone = Arc::clone(&frames);
    let mut cb = CallbackCamera::new(index, requested, move |frame| {
        println!(
            "Captured Single Frame of {} bytes, format: {:?}",
            frame.buffer().len(),
            frame.source_frame_format()
        );

        let decoded = frame.decode_image::<RgbFormat>();
        if decoded.is_err() {
            println!("Error decoding frame");
            return;
        }
        let decoded = decoded.unwrap();

        //previousImage = decoded.clone();
        if decoded.width() == 0 || decoded.height() == 0 {
            println!("Empty Frame");
            return;
        }

        frames_clone.lock().unwrap().push(decoded.clone());
    })
    .unwrap();

    cb.open_stream().unwrap();

    std::thread::sleep(Duration::from_secs(2));

    cb.stop_stream().unwrap();

    let frames_dir = "frames_nokhwa";
    fs::remove_dir_all(frames_dir).ok();
    fs::create_dir_all(frames_dir).expect("Failed to create output directory");
    for (i, frame) in frames.lock().unwrap().iter().enumerate() {
        let frame_path = format!("{}/frame_{:04}.png", frames_dir, i);
        frame.save(frame_path).unwrap();
    }
}
