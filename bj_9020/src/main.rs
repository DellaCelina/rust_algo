use std::fmt::Write;

mod prime {
    const MAX_N: usize = 10_000 + 1;
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
    let tc = get_in();

    let nums: Vec<_> = (0..tc).map(|_| get_in()).collect();

    let max = nums.iter().max().unwrap();
    prime::calc_to(*max);

    let parts = nums.iter().map(|&x| g_parts(x));

    let mut ret = String::new();
    parts.for_each(|v| {
        v.iter().for_each(|&v| write!(ret, "{v} ").unwrap());
        write!(ret, "\n").unwrap();
    });

    println!("{ret}");
}

fn get_in() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}

fn g_parts(n: u32) -> Vec<u32> {
    use std::ops::ControlFlow::*;

    let l_primes: Vec<_> = (2..=n / 2).rev().filter(|&x| prime::is_prime(x)).collect();
    let r_primes: Vec<_> = (n / 2..=n).filter(|&x| prime::is_prime(x)).collect();

    let ret = l_primes.iter().try_for_each(|&l| {
        r_primes.iter().try_for_each(|&r| {
            if l + r == n {
                Break(vec![l, r])
            } else {
                Continue(())
            }
        })
    });

    match ret {
        Break(v) => v,
        _ => vec![],
    }
}
