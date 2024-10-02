use bit_vec::BitVec;
use crc::{Crc, CRC_16_IBM_SDLC};

const PREABLE_LEN: usize = 8;
const SIZE_LEN: usize = 16;
const CRC_LEN: usize = 16;
const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC); // 16-bit CRC

// overhead = 8 bits preamble + 16 bits size + 16 bits CRC = 40 bits
// => 40 bits / 30 fps = 1.33 seconds

pub fn get_preamble() -> BitVec {
    BitVec::from_bytes(&[0b10101010])
}

pub fn encode_package(data: &BitVec) -> BitVec {
    let mut package = BitVec::new();

    // Add preamble
    package.extend(get_preamble().iter());

    // data to bytes
    let data_bytes = data.to_bytes();

    // add size in bytes
    if data_bytes.len() > u16::MAX as usize {
        panic!("Data too large for size field");
    }
    let size = data_bytes.len() as u16;
    package.extend(BitVec::from_bytes(&size.to_be_bytes()).iter());

    // Add data
    package.extend(data.iter());

    // add CRC of data
    //let crc_value = X25.checksum(&package.to_bytes()[8..]);
    let crc_value = X25.checksum(&data_bytes);
    package.extend(BitVec::from_bytes(&crc_value.to_be_bytes()).iter());

    package
}

// BitVec doesn't support slice access
//let size_bytes: [u8; SIZE_BYTES] = [package_bytes[size_index], package_bytes[size_index + 1]];
pub fn decode_package(package_bits: &BitVec) -> Option<BitVec> {
    // find preamble
    let preamble_bits = get_preamble();
    let mut preamble_index = None;
    for i in 0..package_bits.len() {
        let window = package_bits.iter().skip(i).take(PREABLE_LEN);
        if window.eq(preamble_bits.iter()) {
            preamble_index = Some(i);
            break;
        }
    }

    if preamble_index.is_none() {
        println!("Preamble not found");
        return None;
    }

    // read size
    let size_index = preamble_index.unwrap() + PREABLE_LEN;
    if !check_within_bounds(&package_bits, size_index, SIZE_LEN) {
        println!("Size out of bounds");
        return None;
    }
    let size = read_two_bytes(&package_bits, size_index);
    let data_index = size_index + SIZE_LEN;
    let crc_index = data_index + size as usize * 8;

    if !check_within_bounds(&package_bits, crc_index, CRC_LEN) {
        println!("Data out of bounds");
        return None;
    }

    // read data
    let data: BitVec = package_bits
        .iter()
        .skip(data_index)
        .take(size as usize * 8)
        .collect();

    // read CRC
    let crc = read_two_bytes(&package_bits, crc_index);
    // check CRC
    let data_bytes = data.to_bytes();
    let crc_value = X25.checksum(&data_bytes);
    if crc != crc_value {
        println!("CRC mismatch: {} != {}", crc, crc_value);
        return None;
    }

    // debug
    //println!("preamble_index: {}", preamble_bits_index.unwrap());
    //println!("size_index: {}", size_index);
    //println!("data_index_start: {}", data_index);
    //println!("decode size: {}", size);
    //println!("CRC: {} == {}", crc, crc_value);

    Some(data)
}

fn read_two_bytes(data: &BitVec, start: usize) -> u16 {
    let bytes: [u8; 2] = data
        .iter()
        .skip(start)
        .take(16)
        .collect::<BitVec>()
        .to_bytes()
        .try_into()
        .unwrap();
    u16::from_be_bytes(bytes)
}

fn check_within_bounds(data: &BitVec, start: usize, len: usize) -> bool {
    start + len <= data.len()
}
