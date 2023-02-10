fn main() {
    let &[n, k] = get_in().as_slice() else { panic!() };

    let mut students = get_in();
    students.sort();

    println!("{}", students.get((n - k) as usize).unwrap());
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}
