fn main() {
    let num = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    let ranges = [
        (90..=100, 'A'),
        (80..=89, 'B'),
        (70..=79, 'C'),
        (60..=69, 'D'),
    ];

    let ret = ranges
        .into_iter()
        .find(|(range, _)| range.contains(&num))
        .map_or('F', |(_, v)| v);
    println!("{}", ret);
}
