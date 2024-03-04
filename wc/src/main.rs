use std::env;
use std::fs::File;
use std::io::{ self, Read };
use std::io::BufReader;
use std::io::BufRead;

fn main () {
    let args: Vec<String> = env::args().collect();

    let options = Options::new(&args);
    let stats: Stats;

    if options.filename == "stdin".to_string() {
        stats = process_stdin();
    } else {
        stats = process_file(&options.filename);
    }

    show_stats_for_options(&stats, &options);
}

struct Options {
    bytes: bool,
    lines: bool,
    words: bool,
    chars: bool,
    filename: String,
}

impl Options {
    fn new(args: &Vec<String>) -> Options {
        let default = args.len() < 3;
        let mut bytes = default;
        let mut lines = default;
        let mut words = default;
        let mut chars = false;
        let mut filename = "stdin".to_string();

        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-c" => bytes = true,
                "-l" => lines = true,
                "-w" => words = true,
                "-m" => chars = true,
                other => filename = other.to_string(),
            }
        }

        Options { bytes, lines, words, chars, filename }
    }
}

struct Stats {
    bytes: usize,
    lines: usize,
    words: usize,
    chars: usize,
}

fn process_file(filename: &str) -> Stats {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut bytes = 0;
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        lines += 1;
        bytes += line.as_bytes().len() + 1;
        chars += line.chars().count() + 1;
        words += line.split_whitespace().count();
    }

    Stats { bytes, lines, words, chars }
}

fn process_stdin() -> Stats {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let _ = handle.read_to_string(&mut buffer);

    let mut bytes = 0;
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    for line_result in buffer.lines() {
        let line = line_result;

        lines += 1;
        bytes += line.as_bytes().len() + 1;
        chars += line.chars().count() + 1;
        words += line.split_whitespace().count();
    }

    Stats { bytes, lines, words, chars }
}

fn show_stats_for_options(stats: &Stats, options: &Options) {
    if options.lines { print!("   {}", stats.lines); }
    if options.words { print!("   {}", stats.words); }
    if options.bytes { print!("   {}", stats.bytes); }
    if options.chars { print!("   {}", stats.chars); }
    print!("\n");
}

