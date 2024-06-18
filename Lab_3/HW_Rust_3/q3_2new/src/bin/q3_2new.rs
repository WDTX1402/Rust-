fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arghhhh =args.len();

    if arghhhh != 2 {
        println!("Invalid Input");
    } else if args[1].parse::<f32>().is_err() {
        println!("Invalid Input");
    } else {

        let num: i32 = args[1].parse().unwrap_or(0);

        for i in 1..=num {
            for a in 0..(num - i) {
                print!(" ");
            }
            for a in 0..i {
                print!("*");
            }
            println!();
        }

        for i in 1..num {
            for a in 0..i {
                print!(" ");
            }
            for a in 0..(num - i) {
                print!("*");
            }
            println!();
        }
    }
}
