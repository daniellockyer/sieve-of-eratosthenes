use std::{env, io, process};
use std::io::Write;

fn primes_lt(bound: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3usize;

    while num * num <= bound {
        let mut j = num * num;
        while j <= bound {
            primes[j] = false;
            j += num;
        }
        num += 2;
    }

    primes
        .into_iter()
        .enumerate()
        .skip(2)
        .filter_map(|(i, p)| if p {Some(i)} else {None})
        .collect::<Vec<usize>>()
}

fn main() {
    let bound: usize = env::args().nth(1).unwrap_or_else(|| {
        writeln!(io::stderr(), "[!] Please supply bound!").unwrap();
        process::exit(1);
    }).parse().unwrap();

    println!("{:?}", primes_lt(bound));
}
