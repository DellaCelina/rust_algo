struct Time {
    h: u32,
    m: u32,
}

impl Time {
    fn add(&self, op: u32) -> Time {
        let m = self.m + op;
        let h = self.h + m / 60;

        Time {
            h: h % 24,
            m: m % 60,
        }
    }
}

fn main() {
    use std::io::stdin;

    let t = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let nums: Vec<_> = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Time {
            h: nums[0],
            m: nums[1],
        }
    };

    let m = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    let ret = t.add(m);
    println!("{} {}", ret.h, ret.m);
}
