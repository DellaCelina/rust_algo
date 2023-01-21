fn main() {
    let ops = {
        let ops: Vec<_> = (0..2)
            .map(|_| {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input.trim().parse::<u32>().unwrap()
            })
            .collect();
        (ops[0], ops[1])
    };

    let op2_digits = {
        let mut digits = (0..=2)
            .map(|x| {
                let k = 10u32.pow(x);
                (ops.1 / k) % 10
            })
            .collect::<Vec<_>>();
        digits.push(ops.1);
        digits
    };

    op2_digits
        .into_iter()
        .map(|d| ops.0 * d)
        .for_each(|x| println!("{}", x));
}
