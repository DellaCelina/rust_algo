mod count_sort {
    use std::io::{BufWriter, Write};

    const MAX: usize = 10_000 + 1;
    static mut COUNT: [u32; MAX] = [0; MAX];

    pub fn add_cnt(n: usize) {
        unsafe {
            COUNT[n] += 1;
        }
    }

    pub fn get_cnt(n: usize) -> u32 {
        unsafe { COUNT[n] }
    }

    pub fn print() {
        let mut stdout = BufWriter::new(std::io::stdout().lock());
        (0..MAX).for_each(|v| {
            (0..get_cnt(v)).for_each(|_| write!(stdout, "{v}\n").unwrap());
        });
    }
}

fn main() {
    let n = get_in();
    (0..n).map(|_| get_in()).for_each(|v| {
        count_sort::add_cnt(v as usize);
    });

    count_sort::print();
}

fn get_in() -> u32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
