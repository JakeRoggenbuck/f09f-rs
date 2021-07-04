pub fn horizontal_space(amount: usize, ch: char) -> String {
    let mut space: String = String::new();
    for _ in 0..amount {
        space.push(ch);
    }
    return space;
}
