use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::{env::current_dir, fs, path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(long)]
    file: Order,
    #[clap(long)]
    cpu: Order,
}

#[derive(Debug, Clone, ValueEnum)]
enum Order {
    Serial,
    Parallel,
}

fn main() {
    let dir = current_dir().expect("get current dir").join("output");
    fs::create_dir_all(&dir).expect("make sure output exists");
    match Cli::parse() {
        Cli {
            file: Order::Serial,
            cpu: Order::Serial,
        } => file_serial(dir, fib_serial),
        Cli {
            file: Order::Serial,
            cpu: Order::Parallel,
        } => file_serial(dir, fib_parallel),
        Cli {
            file: Order::Parallel,
            cpu: Order::Serial,
        } => file_parallel(dir, fib_serial),
        Cli {
            file: Order::Parallel,
            cpu: Order::Parallel,
        } => file_parallel(dir, fib_parallel),
    }
}

const MAX: u64 = 40;

fn file_serial(dir: PathBuf, fib: impl Fn(u64) -> u64 + Send + Sync) {
    (0..MAX).map(data(dir, fib)).for_each(proc)
}

fn file_parallel(dir: PathBuf, fib: impl Fn(u64) -> u64 + Send + Sync) {
    (0..MAX).into_par_iter().map(data(dir, fib)).for_each(proc)
}

fn data(
    dir: PathBuf,
    fib: impl Fn(u64) -> u64 + Send + Sync,
) -> impl Fn(u64) -> (PathBuf, String) + Send + Sync {
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
