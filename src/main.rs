pub mod day1;
pub mod day2;

use getopts::Options;
use std::env;

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

    match matches.opt_get::<i32>("d") {
        Ok(Some(d)) => match d {
            1 => day1::main(),
            2 => day2::main(),
            d => panic!("Unknown day: {}", d),
        },
        _ => print_usage(&program, opts),
    }
}
