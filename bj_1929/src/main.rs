use std::fmt::Write;

mod prime {
    const MAX_N: usize = 1_000_000 + 1;
    static mut NOT_PRIME: [bool; MAX_N] = [false; MAX_N];

    pub fn set_not_prime(n: u32) {
        unsafe {
            NOT_PRIME[n as usize] = true;
        }
    }

    pub fn is_prime(n: u32) -> bool {
        unsafe { !NOT_PRIME[n as usize] }
    }
}

fn main() {
    let &[m, n] = get_in().as_slice() else { panic!() };
    prime::set_not_prime(1);
    (2..=n).for_each(|v| {
        if prime::is_prime(v) {
            (2..)
                .take_while(|&x| x * v <= n)
                .map(|x| x * v)
                .for_each(prime::set_not_prime);
        }
    });

    let mut ret = String::new();
    (m..=n).for_each(|x| {
        if prime::is_prime(x) {
            write!(ret, "{x}\n").unwrap()
        }
    });

    print!("{ret}");
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}
