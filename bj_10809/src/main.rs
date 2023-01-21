fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    (b'a'..=b'z').for_each(|v| {
        let pos = input
            .as_bytes()
            .iter()
            .position(|&x| x == v)
            .map(|x| x as i32)
            .unwrap_or(-1);
        print!("{} ", pos);
    })
}
