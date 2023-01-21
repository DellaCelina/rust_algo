fn get_in() -> Vec<u32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect()
}

fn main() {
    let mut nums: Vec<_> = (0..28).map(|_| get_in()[0]).collect();
    nums.push(31);
    nums.sort();
    nums.into_iter()
        .scan(1, |state, x| {
            let ret = *state..x;
            *state = x + 1;
            Some(ret)
        })
        .flatten()
        .for_each(|x| println!("{}", x));
}
