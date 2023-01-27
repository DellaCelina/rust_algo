fn main() {
    let mat = get_mat();

    let max = mat.iter().enumerate().fold((0, 0, 0), |st, (idx, v)| {
        let max = v.iter().enumerate().max_by(|&a, &b| a.1.cmp(&b.1)).unwrap();
        if *max.1 > st.0 {
            (*max.1, idx, max.0)
        } else {
            st
        }
    });

    println!("{}", max.0);
    println!("{} {}", max.1 + 1, max.2 + 1);
}

fn get_mat() -> Vec<Vec<i32>> {
    (0..9).map(|_| get_row()).collect()
}

fn get_row() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}
