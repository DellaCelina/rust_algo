fn main() {
    use std::io::stdin;

    let cnt: u32 = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    (0..cnt)
        .map(|_| {
            let nums: Vec<u32> = {
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                input
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            };
            nums[0] + nums[1]
        })
        .for_each(|x| println!("{}", x));
}
