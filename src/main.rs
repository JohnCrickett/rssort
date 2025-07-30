use std::io::Write;

use clap::Parser;
use rand::seq::SliceRandom;
use rand::rng;

#[derive(Parser, Debug)]
#[command(author = "John Crickett", version, about="rssort, a simple sort clone in Rust")]
struct Arguments {
    /// Unique keys.  Suppress all lines that have a key that is equal to an already processed one.
    #[arg(short, action = clap::ArgAction::SetTrue)]
    u: bool,

    /// Use Quicksort
    #[arg(long, action = clap::ArgAction::SetTrue)]
    qsort: bool,

    /// Use Mergesort
    #[arg(long, action = clap::ArgAction::SetTrue)]
    mergesort: bool,

    /// Use random sort
    #[arg(long, action = clap::ArgAction::SetTrue)]
    random_sort: bool,

    files: Vec<String>,
}

fn main() {
    let args = Arguments::parse();
    
    if args.files.is_empty() {
        eprintln!("Error: Please provide a filename as an argument");
        std::process::exit(1);
    }

    if check_flags(args.qsort, args.mergesort, args.random_sort) {
        eprintln!("Error: Only one sorting flag can be used at a time");
        std::process::exit(1);
    }


    let filename = args.files[0].clone();

    let mut contents: Vec<String> = match std::fs::read_to_string(&filename) {
        Ok(content) => content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(String::from)
            .collect(),
        Err(e) => {
            eprintln!("Error reading file {filename}: {e}");
            std::process::exit(1);
        }
    };

    if args.qsort {
        let len = contents.len();
        quicksort(&mut contents, 0, len as isize - 1);
    } else if args.mergesort {
        mergesort(&mut contents);
    } else if args.random_sort {
        contents.shuffle(&mut rng());
    } else {
        contents.sort();
    }

    if args.u {
        contents.dedup();
    }

    for l in contents {
        if let Err(e) = writeln!(std::io::stdout(), "{l}") {
            eprintln!("Error writing to stdout: {e}");
            std::process::exit(1);
        }
    }
}

fn check_flags(quicksort: bool, mergersort: bool, randomsort: bool) -> bool {
    let flag_count = [quicksort, mergersort, randomsort].iter().filter(|&&x| x).count();
    flag_count > 1
}


fn quicksort(arr: &mut Vec<String>, low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        quicksort(arr, low, p - 1);
        quicksort(arr, p + 1, high);
    }
}

fn partition(arr: &mut [String], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut i = low - 1;

    for j in low..high {
        let j = j as usize;
        if arr[j] <= arr[pivot] {
            i += 1;
            arr.swap(i as usize, j);
        }
    }

    let i = i + 1;
    arr.swap(i as usize, pivot);
    i
}

fn mergesort(arr: &mut [String]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    mergesort(&mut left);
    mergesort(&mut right);

    merge(arr, &left, &right);
}

fn merge(arr: &mut [String], left: &[String], right: &[String]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
