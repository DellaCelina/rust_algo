fn main() {
    let golen = [1, 1, 2, 2, 2, 8];
    let inputs = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    };

    golen
        .iter()
        .zip(&inputs)
        .map(|(&g, &i)| g - i)
        .for_each(|v| print!("{} ", v));
}
