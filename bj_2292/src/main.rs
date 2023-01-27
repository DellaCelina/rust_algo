fn get_in() -> u64 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
fn main() {
    let n = get_in() as f64;

    // 1 + (0..=i).sum() * 6 < n <= 1 + (0..=(i+1)).sum() * 6
    // 1 + i*(i-1)*3 < n
    // n > 3i^2 - 3i + 1
    // a = 3, b = -3, c = 1 - n
    // 3 + sqrt(9 - 12(1-n)) / 6
    let i = (3. + (9. - 12. * (1. - n)).sqrt()) / 6.;
    let i = if i == i.floor() {
        (i - 1.).floor()
    } else {
        i.floor()
    };
    let i = i as u32;
    println!("{}", i + 1);
}
