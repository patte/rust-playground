use bit_vec::BitVec;
use crc::{Crc, CRC_8_BLUETOOTH};

const PREABLE_LEN: usize = 8;
const SIZE_LEN: usize = 8;
const CRC_LEN: usize = 8;
const CRC: Crc<u8> = Crc::<u8>::new(&CRC_8_BLUETOOTH); // 8-bit CRC

// overhead = 8 bits preamble + 8 bits size + 8 bits CRC = 24 bits
// => 24 bits / 30 fps = 0.8 seconds
// 1 byte at 30 fps = 0.266 ms
// 1 byte at 60 fps = 0.133 ms

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
    if data_bytes.len() > u8::MAX as usize {
        panic!("Data too large for size field");
    }
    let size = data_bytes.len() as u8;
    package.extend(BitVec::from_bytes(&size.to_be_bytes()).iter());

    // Add data
    package.extend(data.iter());

    // add CRC of data
    let crc_value = CRC.checksum(&data_bytes);
    package.extend(BitVec::from_bytes(&crc_value.to_be_bytes()).iter());

    package
}

// BitVec doesn't support slice access
pub fn decode_package(package_bits: &BitVec) -> Option<BitVec> {
    // find preamble
    let preamble_bits = get_preamble();
    for i in 0..package_bits.len() {
        let window = package_bits.iter().skip(i).take(PREABLE_LEN);
        if window.eq(preamble_bits.iter()) {
            let preamble_index = i;
            let package = decode_package_at_index(&package_bits, preamble_index + PREABLE_LEN);
            if package.is_some() {
                return package;
            }
        }
    }

    return None;
}

fn decode_package_at_index(package_bits: &BitVec, start_index: usize) -> Option<BitVec> {
    // read size
    let size_index = start_index;
    if !check_within_bounds(&package_bits, size_index, SIZE_LEN) {
        println!("Size out of bounds");
        return None;
    }
    let size = read_one_byte(&package_bits, size_index);
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
    let crc = read_one_byte(&package_bits, crc_index);
    // check CRC
    let data_bytes = data.to_bytes();
    let crc_value = CRC.checksum(&data_bytes);
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

fn read_one_byte(data: &BitVec, start: usize) -> u8 {
    let bytes: [u8; 1] = data
        .iter()
        .skip(start)
        .take(8)
        .collect::<BitVec>()
        .to_bytes()
        .try_into()
        .unwrap();
    bytes[0]
}

fn check_within_bounds(data: &BitVec, start: usize, len: usize) -> bool {
    start + len <= data.len()
}
