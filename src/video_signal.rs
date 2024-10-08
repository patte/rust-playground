use bit_vec::BitVec;
use util::message::{decode_message, encode_message, Message};
use util::signal::{decode_package, encode_package};
use util::video::{read_video, write_video};
mod util;

const FPS: u32 = 30;

// send and receive data using video
fn main() {
    let encoded_data = BitVec::from_bytes(&[0b11001110, 0b00110001]);
    let decoded_package = send_receive(&encoded_data);
    println!("Decoded: {:?}", decoded_package);
    println!("");
    assert_eq!(encoded_data, decoded_package);

    let text_message = "https://github.com/patte";
    let encoded_message = BitVec::from_bytes(&text_message.to_string().into_bytes());
    let decoded_package = send_receive(&encoded_message);
    let decoded_message = String::from_utf8(decoded_package.to_bytes()).unwrap();
    println!("Decoded: {:?}", decoded_message);
    println!("");
    assert_eq!(text_message, decoded_message);

    let message = Message {
        id: 1,
        content: String::from("Hello World!"),
    };
    let encoded_message = encode_message(&message);
    let decoded_package = send_receive(&encoded_message);
    let decoded_message = decode_message(&decoded_package);
    println!("Decoded: {:?}", decoded_message);
    assert_eq!(message, decoded_message);
}

fn send_receive(data: &BitVec) -> BitVec {
    let mut package_data = encode_package(data);

    // add bytes to test robustness
    for _ in 0..3 {
        package_data.insert(0, false);
    }
    for _ in 0..3 {
        package_data.push(false);
    }

    write_video(&package_data, FPS, 2, 2);

    let received_data = read_video();

    let decoded_package = decode_package(&received_data).unwrap();

    //println!("Sent Data:       {:?}", package_data);
    //println!("Decoded Package: {:?}", decoded_package);
    //println!("Received data:   {:?}", received_data);
    //println!("Received Package length: {} bits", &received_data.len());
    println!(
        "Size package: {} payload: {}, ratio: {:.3} duration: {:.3}s",
        package_data.len(),
        decoded_package.len(),
        decoded_package.len() as f32 / received_data.len() as f32,
        package_data.len() as f32 / FPS as f32
    );

    decoded_package
}
