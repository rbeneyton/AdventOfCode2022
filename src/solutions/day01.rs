// use itertools::Itertools;

#[inline]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {

        format!("{}", input)
    } else {

        format!("{}", input)
    }
}
