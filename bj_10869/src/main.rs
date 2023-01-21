fn main() {
    use std::io;

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<_> = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let fns: Vec<Box<dyn Fn(f64, f64) -> f64>> = vec![
        Box::new(|a: f64, b: f64| a + b),
        Box::new(|a: f64, b: f64| a - b),
        Box::new(|a: f64, b: f64| a * b),
        Box::new(|a: f64, b: f64| (a / b).floor()),
        Box::new(|a: f64, b: f64| a % b),
    ];

    fns.into_iter().for_each(|f| {
        nums.iter()
            .cloned()
            .reduce(|a, b| f(a, b))
            .map(|x| println!("{}", x));
    });
}
