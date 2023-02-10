fn main() {
    let mut arr: Vec<_> = (0..5).map(|_| get_in()).collect();
    arr.sort();
    let avg = arr
        .clone()
        .into_iter()
        .reduce(|a, b| a + b)
        .map(|v| v / 5)
        .unwrap();
    let mid = arr.get(2).unwrap();

    println!("{avg}");
    println!("{mid}");
}

fn get_in() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
