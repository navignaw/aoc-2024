pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day7;

use getopts::Options;
use std::env;
use std::fs;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("d", "day", "choose day to run", "DAY");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let day = match matches.opt_get::<i32>("d") {
        Ok(Some(d)) => d,
        _ => panic!("Invalid or missing day specified"),
    };

    let input_file = format!("data/day{}.txt", day);
    let input = match fs::read_to_string(input_file.clone()) {
        Ok(s) => s,
        _ => panic!("Invalid or missing file {}", input_file),
    };

    match day {
        1 => day1::main(input),
        2 => day2::main(input),
        3 => day3::main(input),
        4 => day4::main(input),
        5 => day5::main(input),
        7 => day7::main(input),
        d => panic!("Unknown day: {}", d),
    }
}
