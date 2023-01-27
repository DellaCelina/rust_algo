fn main() {
    let tc = get_in()[0];
    let ret: Vec<_> = (0..tc).map(|_| calc_room()).collect();

    ret.iter().for_each(|&x| println!("{x}"));
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}

fn calc_room() -> u32 {
    let &[h, _, n, ..] = get_in().as_slice() else { panic!() };
    let x = (n - 1) / h + 1;
    let y = (n - 1) % h + 1;
    y * 100 + x
}
