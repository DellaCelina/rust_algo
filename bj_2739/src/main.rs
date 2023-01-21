fn main() {
    let num = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    (1..10)
        .map(|x| format!("{} * {} = {}", num, x, num * x))
        .for_each(|s| println!("{}", s));
}
