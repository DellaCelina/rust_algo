fn main() {
    let v: u32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    println!("{}", (1..=v).into_iter().product::<u32>());
}