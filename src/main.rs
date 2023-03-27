use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::path;
use std::process;

mod map_reduce;

struct CLI {
    data_in: path::PathBuf,
    data_out: path::PathBuf,
}

impl CLI {
    fn build(args: Vec<String>) -> Result<CLI, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let data_in = args[1].clone();
        let data_out = args[2].clone();
        Ok(Self {
            data_in: path::PathBuf::from(data_in),
            data_out: path::PathBuf::from(data_out),
        })
    }
}

// This is the `main` thread
fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    // args gets moved and consumed
    let conf = CLI::build(args).unwrap_or_else(|err| {
        eprintln!(
            "Problem parsing arguments: {err}.
Usage: [data_in] [data_out]"
        );
        process::exit(1);
    });

    // This is our data to process.
    // We will calculate the sum of all digits via a threaded  map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.

    let data = fs::read_to_string(&conf.data_in).unwrap_or_else(|err| {
        eprintln!("Error opening the file {err}");
        process::exit(1)
    });

    // Make a vector to hold the child-threads which we will spawn.

    // split our data into segments for individual calculation
    // each chunk will be a reference (&str) into the actual data

    // let chunked_data = data.clone().as_str().split_whitespace();
    // NOTE: (aver) As suggested by ChatGPT:
    // This converts the iterator of &str references into an owned vector of String values.
    // The map call is necessary to convert each &str into a String so that they are owned by the vector.
    let chunked_data: Vec<String> = data
        .clone()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // We'll use a mutex to ensure that only one thread is writing to the output file at a time.
    let mut output_file = fs::File::create(&conf.data_out)?;

    let children_threads = map_reduce::map(chunked_data);
    let final_result = map_reduce::reduce(children_threads);

    println!("Final sum result: {}", final_result);
    output_file.write_fmt(format_args!("Final sum result: {}\n", final_result));

    Ok(())
}
