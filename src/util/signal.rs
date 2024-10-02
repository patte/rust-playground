use bit_vec::BitVec;

pub fn get_preamble() -> BitVec {
    BitVec::from_bytes(&[0b10101010])
}

pub fn wrap_data(data: BitVec) -> BitVec {
    let mut wrapped_data = BitVec::new();
    wrapped_data.extend(get_preamble().iter());
    wrapped_data.extend(data.iter());
    wrapped_data
}

pub fn unwrap_data(wrapped_data: &BitVec) -> BitVec {
    wrapped_data.iter().skip(8).map(|bit| bit).collect()
}
