fn get_in() -> Vec<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn main() {
    let tc: u32 = get_in()[0].parse().unwrap();

    for _ in 0..tc {
        let input = get_in();
        let [n, s, ..] = input.as_slice() else { panic!() };
        let n = n.parse().unwrap();
        let ret: String = s
            .chars()
            .into_iter()
            .flat_map(|c| {
                let mut v: Vec<char> = Vec::new();
                for _ in 0..n {
                    v.push(c);
                }
                v
            })
            .collect();
        println!("{}", ret);
    }
}
