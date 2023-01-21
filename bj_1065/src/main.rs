use std::collections::BTreeSet;
fn main() {
    let num: i32 = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    let all_nums = |st, diff| {
        (0..)
            .scan(0, |state, _| {
                if *state == 0 {
                    if st <= num {
                        *state = st;
                        return Some(st);
                    } else {
                        return None;
                    }
                }

                let next_d = (*state % 10) + diff;
                if (0..10).contains(&next_d) {
                    let next = *state * 10 + next_d;
                    if next <= num {
                        *state = next;
                        Some(next)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let ret = (1..=9)
        .flat_map(move |st| {
            (-9..=9)
                .flat_map(|diff| all_nums(st, diff))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .count();
    println!("{}", ret);
}
