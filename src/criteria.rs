pub fn matches(addr: &[u8]) -> bool {
    addr[..3].iter().all(|b| *b == 0) // && addr[3] <= 4
}
