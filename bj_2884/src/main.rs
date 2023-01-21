struct Time {
    h: u32,
    m: u32,
}

impl Time {
    fn alarm(&self, sh: u32) -> Time {
        let m = self.m + 60 - sh;
        let h = self.h + m / 60 + 23;

        Time {
            h: h % 24,
            m: m % 60,
        }
    }
}

fn main() {
    let t = {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<_> = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        Time {
            h: nums[0],
            m: nums[1],
        }
    };

    let alarm = t.alarm(45);
    println!("{} {}", alarm.h, alarm.m);
}
