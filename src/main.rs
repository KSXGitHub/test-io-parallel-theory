use clap::Parser;
use rayon::prelude::*;
use std::{env::current_dir, fs, path::PathBuf};

#[derive(Debug, Parser)]
enum Cli {
    Serial,
    Parallel,
}

fn main() {
    let dir = current_dir().expect("get current dir").join("output");
    fs::create_dir_all(&dir).expect("make sure output exists");
    match Cli::parse() {
        Cli::Serial => serial(dir),
        Cli::Parallel => parallel(dir),
    }
}

const MAX: u32 = 500;

fn data(dir: PathBuf) -> impl Fn(u32) -> (PathBuf, String) {
    move |num| {
        let path = dir.join(format!("{num}.txt"));
        let content = (0..num)
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        (path, content)
    }
}

fn proc((path, content): (PathBuf, String)) {
    fs::write(path, content).expect("write a file");
}

fn serial(dir: PathBuf) {
    (0..MAX).map(data(dir)).for_each(proc)
}

fn parallel(dir: PathBuf) {
    (0..MAX).into_par_iter().map(data(dir)).for_each(proc)
}
