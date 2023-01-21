use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: Vec<_> = input
        .trim()
        .chars()
        .into_iter()
        .map(|c| c.to_ascii_uppercase())
        .collect();

    let mut hash = HashMap::new();
    input.iter().for_each(|&c| {
        if let Some(x) = hash.get_mut(&c) {
            *x += 1;
        } else {
            hash.insert(c, 0);
        }
    });

    let max = hash.iter().max_by(|&op0, &op1| op0.1.cmp(&op1.1)).unwrap();

    let max_cnt = hash.iter().filter(|&t| t.1 == max.1).count();
    if max_cnt == 1 {
        println!("{}", max.0);
    } else {
        println!("?");
    }
}
