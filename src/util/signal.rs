use bit_vec::BitVec;

pub fn get_preamble() -> BitVec {
    BitVec::from_bytes(&[0b10101010])
}

pub fn encode_package(data: BitVec) -> BitVec {
    let mut package = BitVec::new();
    // Add preamble
    package.extend(get_preamble().iter());
    // add 2 bytes size
    if data.to_bytes().len() > u16::MAX as usize {
        panic!("Data too large for size field");
    }
    let size = data.to_bytes().len() as u16;
    println!("encode size: {}", size);
    package.extend(BitVec::from_bytes(&size.to_be_bytes()).iter());
    // Add data
    package.extend(data.iter());
    package
}

const SIZE_LEN: usize = 16;

// BitVec doesn't support slice access
//let size_bytes: [u8; SIZE_BYTES] = [package_bytes[size_index], package_bytes[size_index + 1]];
pub fn decode_package(package_bits: &BitVec) -> Option<BitVec> {
    // find preamble
    let preamble_bits = get_preamble();
    let mut preamble_bits_index = None;
    for i in 0..package_bits.len() {
        let window = package_bits.iter().skip(i).take(preamble_bits.len());
        if window.eq(preamble_bits.iter()) {
            preamble_bits_index = Some(i);
            break;
        }
    }

    if preamble_bits_index.is_none() {
        println!("Preamble not found");
        return None;
    }

    let size_index = preamble_bits_index.unwrap() + preamble_bits.len();
    let data_index = size_index + SIZE_LEN;

    // read size
    let size_be_bytes: [u8; SIZE_LEN / 8] = package_bits
        .iter()
        .skip(size_index)
        .take(SIZE_LEN)
        .collect::<BitVec>()
        .to_bytes()
        .try_into()
        .unwrap();
    let size = u16::from_be_bytes(size_be_bytes);

    //print all indices
    println!("preamble_index: {}", preamble_bits_index.unwrap());
    println!("size_index: {}", size_index);
    println!("data_index_start: {}", data_index);
    println!("decode size: {}", size);

    Some(
        package_bits
            .iter()
            .skip(data_index)
            .take(size as usize * 8)
            .collect(),
    )
}
