struct Point(i32, i32);

fn main() {
    let p = {
        let get_v = || -> i32 {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        };

        Point(get_v(), get_v())
    };

    match p {
        Point(x, y) if x > 0 && y > 0 => println!("1"),
        Point(x, y) if x < 0 && y > 0 => println!("2"),
        Point(x, y) if x < 0 && y < 0 => println!("3"),
        Point(x, y) if x > 0 && y < 0 => println!("4"),
        _ => println!("0"),
    }
}
