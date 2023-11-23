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

const MAX: u32 = 300;

fn proc(dir: PathBuf) -> impl Fn(u32) {
    move |num| {
        let path = dir.join(format!("{num}.txt"));
        let content = num.to_string();
        fs::write(path, content).expect("write a file");
    }
}

fn serial(dir: PathBuf) {
    (0..MAX).for_each(proc(dir))
}

fn parallel(dir: PathBuf) {
    (0..MAX).into_par_iter().for_each(proc(dir))
}
