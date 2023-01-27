use std::fmt::Write;
fn main() {
    let &[r, _c] = get_in().as_slice() else { panic!() };

    let mats = (0..2)
        .map(|_| (0..r).map(|_| get_in()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let out = mats
        .into_iter()
        .reduce(|a, b| {
            a.into_iter()
                .zip(b.into_iter())
                .map(|(ar, br)| {
                    ar.into_iter()
                        .zip(br.into_iter())
                        .map(|(ac, bc)| ac + bc)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .unwrap();

    let mut ret = String::new();
    out.iter().for_each(|v| {
        v.iter().for_each(|x| write!(ret, "{x} ").unwrap());
        write!(ret, "\n").unwrap();
    });
    print!("{ret}");
}

fn get_in() -> Vec<u32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.split_ascii_whitespace().flat_map(str::parse).collect()
}
