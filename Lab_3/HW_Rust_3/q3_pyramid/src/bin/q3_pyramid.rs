fn main() {
    let args: Vec<String> = std::env::args().collect();
    let pain = args.len();

    if pain != 2 {
        println!("Invalid Input");
    } else if args[1].parse::<f32>().is_err() {
        println!("Invalid Input");
    } else {
        let num: i32 = args[1].parse().unwrap_or(0);
        let mut i: i32 = 1;

        while i <= num {
            let mut yee = 1;
            while yee <= i {
                print!("*");
                yee += 1;
            }
            println!();
            i += 1;
        }

        i = num - 1;
        while i >= 1 {
            let mut yaa = 1;
            while yaa <= i {
                print!("*");
                yaa += 1;
            }
            println!();
            i -= 1;
        }
    }
}
