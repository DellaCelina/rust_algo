use std::io::stdin;
use std::ops::ControlFlow;
fn main() {
    let n: u32 = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    let cnt = (0..n)
        .map(|_| {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input
                .trim()
                .as_bytes()
                .into_iter()
                .map(|&v| v - b'a')
                .collect::<Vec<u8>>()
        })
        .map(|v| {
            const MAP_N: usize = (b'z' - b'a' + 1) as usize;
            let mut g_map = [0u8; MAP_N];
            let mut skip = std::u8::MAX;

            v.iter()
                .try_for_each(|&x| {
                    if x == skip {
                        return ControlFlow::Continue(());
                    }
                    let g = &mut g_map[x as usize];
                    if *g == 0 {
                        *g = 1;
                        skip = x;
                        ControlFlow::Continue(())
                    } else {
                        ControlFlow::Break(())
                    }
                })
                .is_continue()
        })
        .filter(|&b| b)
        .count();

    println!("{}", cnt);
}
