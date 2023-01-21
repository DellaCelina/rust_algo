fn get_in() -> Vec<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .flat_map(str::parse::<i32>)
        .collect()
}

fn main() {
    let num = get_in()[0];

    for i in 0..num {
        let mut ret = String::new();
        for _ in 0..(num - i - 1) {
            ret.push(' ');
        }

        for _ in 0..=i {
            ret.push('*');
        }
        println!("{}", ret);
    }
}
