fn main() {
    let v = get_in();

    let n = v / 5;
    let resd = v % 5;
    let (v_5, v_3) = match resd {
        0 | 3 => (n, resd / 3),
        2 => (n - 2, (resd + 10) / 3),
        _ => (n - 1, (resd + 5) / 3),
    };

    if v_5 < 0 {
        println!("-1");
    } else {
        println!("{}", v_5 + v_3);
    }
}

fn get_in() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
