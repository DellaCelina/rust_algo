fn main() {
    let m = get_in();
    let n = get_in();

    let mut iter: Box<dyn Iterator<Item = u32>> = Box::new((2..).into_iter());

    let mut nums = Vec::new();
    loop {
        let v = iter.next().unwrap();
        if v > n {
            break;
        }
        if v >= m {
            nums.push(v);
        }
        iter = Box::new(iter.filter(move |&x| x % v != 0));
    }

    if nums.is_empty() {
        println!("-1");
    } else {
        let sum = nums.iter().sum::<u32>();
        let min = nums.iter().min().unwrap();
        println!("{sum}");
        println!("{min}");
    }
}

fn get_in() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
