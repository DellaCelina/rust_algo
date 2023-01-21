use std::fmt::Write;

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

    let mut ret = String::new();
    for i in 0..num {
        writeln!(
            ret,
            "Case #{i}: {v}",
            i = i + 1,
            v = get_in().into_iter().reduce(|a, b| a + b).unwrap()
        )
        .unwrap();
    }

    println!("{}", ret);
}
