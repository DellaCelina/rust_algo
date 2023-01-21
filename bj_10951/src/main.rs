use std::io::stdin;

fn main() {
    let lines = stdin().lines().flatten();

    lines
        .map(|x| {
            x.split_ascii_whitespace()
                .flat_map(|s| s.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .flat_map(|a| a.into_iter().reduce(|a, b| a + b))
        .for_each(|x| println!("{}", x));
}
