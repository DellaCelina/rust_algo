const RANGE: u32 = 10000;

fn d(n: u32) -> u32 {
    let digits_sum = (0..)
        .scan(n, |st, _| {
            let ret = *st;
            *st = *st / 10;
            Some(ret)
        })
        .take_while(|&x| x != 0)
        .map(|x| x % 10)
        .sum::<u32>();
    n + digits_sum
}

fn main() {
    let mut ds = (0..)
        .map(|x| d(x))
        .take_while(|&x| x <= RANGE + 100)
        .collect::<Vec<_>>();

    ds.sort();

    ds.into_iter()
        .scan(1, |st, v| {
            let ret = *st..v;
            *st = v + 1;
            Some(ret)
        })
        .flatten()
        .take_while(|&x| x <= RANGE)
        .for_each(|x| println!("{}", x));
}
