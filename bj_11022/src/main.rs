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
    (0..num).for_each(|i| {
        let ops = get_in();
        writeln!(
            ret,
            "Case #{i}: {op0} + {op1} = {v}",
            i = i + 1,
            op0 = ops[0],
            op1 = ops[1],
            v = ops[0] + ops[1],
        )
        .unwrap();
    });

    println!("{}", ret);
}
