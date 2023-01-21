fn main() {
    let inputs = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
            .split_ascii_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>()
    };

    type FnType = Box<dyn Fn(f64, f64, f64) -> f64>;
    let fns: [FnType; 4] = [
        Box::new(|a, b, c| (a + b) % c),
        Box::new(|a, b, c| ((a % c) + (b % c)) % c),
        Box::new(|a, b, c| (a * b) % c),
        Box::new(|a, b, c| ((a % c) * (b % c)) % c),
    ];

    fns.into_iter()
        .map(|f| {
            let &[a, b, c] = inputs.as_slice() else { return 0_f64 };
            f(a, b, c)
        })
        .for_each(|x| println!("{}", x));
}
