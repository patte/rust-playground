use util::message::{decode_message, encode_message, Message};
use util::signal::{decode_package, encode_package};
use util::video::{read_video, write_video};
mod util;

fn main() {
    let fps = 30;

    let message = Message {
        id: 1,
        content: String::from("Hello World!"),
    };
    let encoded_message = encode_message(&message);

    let mut package_data = encode_package(encoded_message);

    // add bytes to test robustness
    for _ in 0..30 {
        package_data.insert(0, false);
    }
    for _ in 0..80 {
        package_data.push(false);
    }

    write_video(&package_data, fps, 2, 2);

    let received_data = read_video();

    let decoded_package = decode_package(&received_data).unwrap();
    println!("Decoded Package: {:?}", decoded_package);
    let decoded = decode_message(&decoded_package);

    println!("Sent Data:       {:?}", package_data);
    println!("Received data:   {:?}", received_data);
    println!("Decoded: {:?}", decoded);

    let test: u8 = 0b11111111;
    println!("test: {}", test);
}
