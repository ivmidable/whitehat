pub fn parse_pubkey(slice: &[u8]) -> [u8; 32] {
    slice.try_into().expect("slice with incorrect length")
}
