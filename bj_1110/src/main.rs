fn cycle(num: u32) -> u32 {
    let n0 = num % 10;
    (n0 * 10) + ((n0 + num / 10) % 10)
}

fn main() {
    let num: u32 = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut ret = input.trim().parse().unwrap();
        if ret < 10 {
            ret *= 10;
        }
        ret
    };

    let ret = (0..)
        .scan(num, |a, _| {
            *a = cycle(*a);
            Some(*a)
        })
        .position(|x| x == num)
        .unwrap()
        + 1;
    println!("{}", ret);
}
