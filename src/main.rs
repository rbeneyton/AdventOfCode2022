use clap::{
    command,
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    Parser
};
use std::time::Instant;

// TODO: find a way to avoid crate name here
use aoc_2022::*;

#[derive(Parser)]
#[command(name = crate_name!())]
#[command(author = crate_authors!(", "))]
#[command(version = crate_version!())]
#[command(about = crate_description!())]
pub struct Options {
    /// day
    #[clap(short, long, default_value = "1")]
    day: Day,
    /// part
    #[clap(short, long, default_value = "1")]
    part: u8,
    /// session cookie
    #[clap(short, long, default_value = "unset")]
    session: String,
    /// bench all until the given day
    #[clap(short, long)]
    bench: bool,
}

fn main() {
    let options = Options::parse();

    if options.bench {
        for day in (1..=25).filter(|d| ![17, 22].contains(d)) {
            let data = get_data(day, &options.session);
            for part in [/*1,*/ 2] {
                let start = Instant::now();
                let _res = solve(day, part, &data);
                let elapsed = start.elapsed();
                println!("day {:2} part {} elapsed: {:10}µs", day, part, elapsed.as_micros());
            }
        }
    } else {
        let data = get_data(options.day, &options.session);
        let res = solve(options.day, options.part, &data);
        println!("day {} part {} solve: {}", options.day, options.part, res)
    }
}
