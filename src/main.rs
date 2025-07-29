use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "John Crickett", version, about="rssort, a simple sort clone in Rust")]
struct Arguments {
    /// Number the output lines, starting at 1.
    #[arg(short, action = clap::ArgAction::SetTrue)]
    u: bool,

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

    contents.sort();

    if args.u {
        contents.dedup()
    }

    for i in contents {
        print!("{}\n", i);
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
