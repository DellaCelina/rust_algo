fn get_in() -> Vec<f64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect()
}

fn main() {
    let nb_tc = get_in()[0] as u32;
    let nums: Vec<_> = (0..nb_tc)
        .map(|_| get_in().into_iter().skip(1).collect::<Vec<_>>())
        .collect();
    let avgs = nums.iter().map(|v| v.iter().sum::<f64>() / v.len() as f64);

    nums.iter()
        .zip(avgs)
        .map(|(v, avg)| {
            let over_avg = v.iter().filter(|&x| *x > avg).count();

            over_avg as f64 / v.len() as f64 * 100.0
        })
        .for_each(|x| println!("{:.3}%", x));
}
