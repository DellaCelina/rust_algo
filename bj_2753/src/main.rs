fn main() {
    let num: u32 = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    match num {
        x if x % 400 == 0 => println!("1"),
        x if x % 100 == 0 => println!("0"),
        x if x % 4 == 0 => println!("1"),
        _ => println!("0"),
    }
}
