fn main() {
    let tc = get_in();

    (0..tc).map(|_| calc_out()).for_each(|v| println!("{v}"));
}

fn get_in() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn calc_out() -> u32 {
    let k = get_in();
    let n = get_in();

    let zero_floor = 1..;
    let mut it: Box<dyn Iterator<Item = u32>> = Box::new(zero_floor.into_iter());

    for _ in 0..k {
        it = Box::new(it.scan(0, |st, v| {
            *st = *st + v;
            Some(*st)
        }));
    }

    it.nth((n - 1) as usize).unwrap()
}
