use util::message::{decode_message, encode_message, Message};
use util::signal::{unwrap_data, wrap_data};
use util::video::{read_video, write_video};
mod util;

fn main() {
    let width = 2;
    let height = 2;
    let fps = 30;

    let message = Message {
        id: 1,
        content: String::from("Let there be light!"),
    };
    let encoded = encode_message(&message);

    let data = wrap_data(encoded);

    write_video(&data, fps, width, height);

    let received_data = read_video();

    let decoded = decode_message(&unwrap_data(&received_data));

    println!("Data: {:?}", data);
    println!("bits: {:?} bytes: {:?}", data.len(), data.to_bytes().len());
    println!("Received data: {:?}", received_data);
    println!("Decoded: {:?}", decoded);
}
