fn main() {
    let num = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    let ret = (0..=num).fold(0, |acc, x| acc + x);
    println!("{}", ret);
}
