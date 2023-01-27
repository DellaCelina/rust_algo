use std::fmt::Write;

mod prime {
    const MAX_N: usize = 123_456 * 2 + 1;
    static mut NOT_PRIME: [bool; MAX_N] = [false; MAX_N];

    fn set_not_prime(n: u32) {
        unsafe {
            NOT_PRIME[n as usize] = true;
        }
    }

    pub fn is_prime(n: u32) -> bool {
        unsafe { !NOT_PRIME[n as usize] }
    }

    pub fn calc_to(n: u32) {
        set_not_prime(1);
        (2..=n).for_each(|v| {
            if is_prime(v) {
                (2..)
                    .take_while(|&x| x * v <= n)
                    .map(|x| x * v)
                    .for_each(set_not_prime);
            }
        });
    }
}

fn main() {
    let nums: Vec<_> = (0..)
        .into_iter()
        .map(|_| {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim().parse::<u32>().unwrap()
        })
        .take_while(|&x| x != 0)
        .collect();

    let max = nums.iter().max().unwrap();

    prime::calc_to(*max * 2);

    let mut ret = String::new();
    nums.iter()
        .map(|&v| {
            ((v + 1)..=(2 * v)).fold(0, |acc, x| if prime::is_prime(x) { acc + 1 } else { acc })
        })
        .for_each(|x| write!(ret, "{x}\n").unwrap());
    print!("{ret}");
}
