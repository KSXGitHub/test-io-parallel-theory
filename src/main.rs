use clap::Parser;
use rayon::prelude::*;
use std::{env::current_dir, fs, path::PathBuf};

#[derive(Debug, Parser)]
enum Cli {
    Serial,
    Parallel,
    Mixed,
}

fn main() {
    let dir = current_dir().expect("get current dir").join("output");
    fs::create_dir_all(&dir).expect("make sure output exists");
    match Cli::parse() {
        Cli::Serial => serial(dir),
        Cli::Parallel => parallel(dir),
        Cli::Mixed => mixed(dir),
    }
}

const MAX: u64 = 40;

fn data(dir: PathBuf, fib: impl Fn(u64) -> u64) -> impl Fn(u64) -> (PathBuf, String) {
    move |num| {
        let path = dir.join(format!("{num}.txt"));
        let content = (0..num)
            .map(|x| fib(x).to_string())
            .collect::<Vec<_>>()
            .join("\n");
        (path, content)
    }
}

fn proc((path, content): (PathBuf, String)) {
    fs::write(path, content).expect("write a file");
}

fn serial(dir: PathBuf) {
    (0..MAX).map(data(dir, fib_serial)).for_each(proc)
}

fn parallel(dir: PathBuf) {
    (0..MAX)
        .into_par_iter()
        .map(data(dir, fib_parallel))
        .for_each(proc)
}

fn mixed(dir: PathBuf) {
    (0..MAX).map(data(dir, fib_parallel)).for_each(proc)
}

fn fib_serial(num: u64) -> u64 {
    match num {
        0 | 1 => num,
        _ => fib_serial(num - 1) + fib_serial(num - 2),
    }
}

fn fib_parallel(num: u64) -> u64 {
    match num {
        0 | 1 => num,
        _ => [num - 2, num - 1].into_par_iter().map(fib_parallel).sum(),
    }
}
