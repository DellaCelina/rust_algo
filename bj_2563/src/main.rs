mod canvas {
    const MAX_N: usize = 100;
    const PAPER_SIZE: u32 = 10;
    static mut CANVAS: [[u8; MAX_N]; MAX_N] = [[0; MAX_N]; MAX_N];

    pub fn attach_paper(p: (u32, u32)) {
        (p.0..p.0 + PAPER_SIZE).for_each(|x| {
            (p.1..p.1 + PAPER_SIZE).for_each(|y| unsafe {
                CANVAS[x as usize][y as usize] += 1;
            })
        })
    }

    pub fn get_cnt() -> usize {
        unsafe {
            CANVAS
                .iter()
                .map(|r| r.iter().filter(|&&v| v != 0).count())
                .sum::<usize>()
        }
    }
}

fn main() {
    let tc = get_in()[0];
    let papers: Vec<_> = (0..tc)
        .map(|_| {
            let &[w, h] = get_in().as_slice() else { panic!() };
            (w, h)
        })
        .collect();

    papers.into_iter().for_each(canvas::attach_paper);
    println!("{}", canvas::get_cnt());
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}
