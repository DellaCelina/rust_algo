fn main() {
    let dial_alpha = [
        ('A'..='C'),
        ('D'..='F'),
        ('G'..='I'),
        ('J'..='L'),
        ('M'..='O'),
        ('P'..='S'),
        ('T'..='V'),
        ('W'..='Z'),
    ];

    let dial_map: Vec<_> = dial_alpha.into_iter().zip(3..=10).collect();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let t = input
        .chars()
        .map(|c| {
            dial_map
                .iter()
                .find(|t| t.0.contains(&c))
                .map(|t| t.1)
                .unwrap()
        })
        .sum::<i32>();
    println!("{}", t);
}
