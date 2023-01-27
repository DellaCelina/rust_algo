fn main() {
    let _n = get_in();
    let nums = get_in();

    let mut iter: Box<dyn Iterator<Item = u32>> = Box::new((2..).into_iter());

    let max = nums.iter().max().unwrap();
    let mut cnt = 0;
    loop {
        let v = iter.next().unwrap();
        if nums.contains(&v) {
            cnt += 1;
        }
        if v >= *max {
            break;
        }
        iter = Box::new(iter.filter(move |&x| x % v != 0));
    }

    println!("{cnt}");
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}
