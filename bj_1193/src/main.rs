fn get_in() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn main() {
    let n = get_in() as f64;

    let i = (1. + (1. - 4. * (1. - 2. * n)).sqrt()) / 2.;
    let i = i.floor() as u32;

    let residual = 1 + (0..i).sum::<u32>();
    let residual = n as u32 - residual;
    let residual = if i % 2 == 0 {
        residual
    } else {
        i - residual - 1
    };

    let b = i - residual;
    let a = i + 1 - b;
    println!("{}/{}", a, b);
}
