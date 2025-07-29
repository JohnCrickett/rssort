use clap::Parser;

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

    files: Vec<String>,
}

fn main() {
    // stop "failed printing to stdout: Broken pipe (os error 32)" when used with head
    reset_sigpipe();

    let args = Arguments::parse();
    
    if args.files.len() < 1 {
        eprintln!("Error: Please provide a filename as an argument");
        std::process::exit(1);
    }

    if args.qsort && args.mergesort {
        eprintln!("Error: Please provide only one sort method");
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
            eprintln!("Error reading file {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    if args.qsort {
        let len = contents.len();
        quicksort(&mut contents, 0, len as isize - 1);
    } else if args.mergesort {
        mergesort(&mut contents);
    } else {
        contents.sort();
    }

    if args.u {
        contents.dedup()
    }

    for i in contents {
        print!("{}\n", i);
    }
}

fn quicksort(arr: &mut Vec<String>, low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        quicksort(arr, low, p - 1);
        quicksort(arr, p + 1, high);
    }
}

fn partition(arr: &mut Vec<String>, low: isize, high: isize) -> isize {
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

fn mergesort(arr: &mut Vec<String>) {
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

fn merge(arr: &mut Vec<String>, left: &[String], right: &[String]) {
    let mut i = 0; // Index for left array
    let mut j = 0; // Index for right array
    let mut k = 0; // Index for merged array

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

    // Copy remaining elements from left array, if any
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    // Copy remaining elements from right array, if any
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

#[cfg(unix)]
fn reset_sigpipe() {
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
}

#[cfg(not(unix))]
fn reset_sigpipe() {
    // no-op
}
