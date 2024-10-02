fn to_hamming(encoded: &Vec<u8>) -> Vec<u8> {
    // Split each byte into two 4-bit nibbles
    let mut encoded_hamming: Vec<u8> = Vec::new();
    for byte in encoded {
        let d1 = byte >> 4;
        let d2 = byte & 0x0F;
        encoded_hamming.push(hamming_encode(d1));
        encoded_hamming.push(hamming_encode(d2));
    }
    encoded_hamming
}

// Hamming(7,4) decoding with single-bit error correction
fn hamming_decode(encoded: u8) -> (u8, bool) {
    let p1 = (encoded >> 6) & 1;
    let p2 = (encoded >> 5) & 1;
    let d1 = (encoded >> 4) & 1;
    let p4 = (encoded >> 3) & 1;
    let d2 = (encoded >> 2) & 1;
    let d3 = (encoded >> 1) & 1;
    let d4 = (encoded >> 0) & 1;

    // Check the parity bits
    let c1 = p1 ^ d1 ^ d2 ^ d4;
    let c2 = p2 ^ d1 ^ d3 ^ d4;
    let c4 = p4 ^ d2 ^ d3 ^ d4;
    let error_position = (c4 << 2) | (c2 << 1) | c1;

    let mut corrected = encoded;
    let error_detected = error_position != 0;

    // Correct the bit if an error was detected
    if error_detected {
        corrected ^= 1 << (7 - error_position);
    }

    // Return the decoded data and whether an error was corrected
    let decoded_data = (corrected & 0x1) | ((corrected >> 1) & 0x6) | ((corrected >> 3) & 0x8);
    (decoded_data, error_detected)
}

// Encode a 4-bit data block into a 7-bit Hamming code
fn hamming_encode(data: u8) -> u8 {
    let d1 = (data >> 3) & 1;
    let d2 = (data >> 2) & 1;
    let d3 = (data >> 1) & 1;
    let d4 = (data >> 0) & 1;

    // Calculate parity bits
    let p1 = d1 ^ d2 ^ d4;
    let p2 = d1 ^ d3 ^ d4;
    let p4 = d2 ^ d3 ^ d4;

    // Return 7-bit Hamming code
    (p1 << 6) | (p2 << 5) | (d1 << 4) | (p4 << 3) | (d2 << 2) | (d3 << 1) | d4
}
