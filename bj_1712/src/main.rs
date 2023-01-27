fn main() {
    let nums: Vec<_> = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.split_ascii_whitespace()
            .flat_map(str::parse::<f64>)
            .collect()
    };

    // A + N * B < C * N
    // A < (C - B) * N
    // A / (C - B) < N

    let c_b = nums[2] - nums[1];
    let ret = if c_b <= 0_f64 {
        -1
    } else {
        ((nums[0] + c_b) / c_b).floor() as i32
    };

    println!("{}", ret);
}
