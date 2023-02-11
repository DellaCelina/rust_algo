const fn fibo(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fibo(n-1) + fibo(n-2),
    }
}

const fn fibo_arr<const N: usize>() -> [u32; N] {
    let mut i: usize = 0;
    let mut ret: [u32; N] = [0; N];
    while i < N {
        ret[i] = fibo(i as u32);
        i += 1;
    }
    ret
}

const FIBOS: [u32; 21] = fibo_arr::<21>();

fn main() {
    let n: u32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };

    println!("{}", FIBOS[n as usize]);
}
