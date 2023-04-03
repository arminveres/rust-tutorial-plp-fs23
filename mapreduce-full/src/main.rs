use std::env;
use std::fs;
use std::io::{self, prelude::*};
use std::path;
use std::process;

mod map_reduce;

/// Struct to hold paths, using PathBuf for OS independent path parsing
struct CLI {
    data_in: path::PathBuf,
    data_out: path::PathBuf,
}

impl CLI {
    /// Takes command line arguments and fills the struct with the paths
    pub fn build(args: Vec<String>) -> Result<CLI, &'static str> {
        // 'static indicates, that the value lives for the whole lifetime of the application.
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Use Strings for easier handling. We could use &str, string slices, though they would
        // require lifetimes. For smaller applications this tradeoff is worth it.
        let data_in = args[1].clone();
        let data_out = args[2].clone();

        // Return a new object (on the stack) wrapped in an OK
        Ok(Self {
            data_in: path::PathBuf::from(data_in),
            data_out: path::PathBuf::from(data_out),
        })
    }
}

/// This is the `main` thread
// In contrast to e.g. C we can return other values from Main than 'void' or 'int', in this case
// it's a type of 'io::Result<()>, since if we fail to create file at line 95, we are returning
// that error. Otherwise if successful we return an empty OK(()) to indicate successful program
// run.
fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    // args gets moved and consumed
    let conf = CLI::build(args).unwrap_or_else(|err| {
        eprintln!(
            // 'eprintln' prints to 'stderr'
            "Problem parsing arguments: {err}.
Usage: [data_in] [data_out]"
        );
        // We can return the application with an exit code.
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

    // As suggested by ChatGPT:
    // This converts the iterator of &str references into an owned vector of String values.
    // The map call is necessary to convert each &str into a String so that they are owned by the vector.
    // Otherwise the references will be dropped before the threads finish.
    let chunked_data = data
        .split_whitespace()
        .map(|s| s.to_string());

    let children_threads = map_reduce::map(chunked_data);
    let final_result = map_reduce::reduce(children_threads);

    // We create a mutable output_file to which we will write, which necessitates for it to be
    // mutable.
    // In case the file creation fails, we use the '?' operator to instantly return an Err() from
    // the application. If used in a separate function, it would just simply return the error for
    // the caller to handle.
    let mut output_file = fs::File::create(&conf.data_out)?;

    println!("Final sum result: {}", final_result); // expects a literal string, not a 'String',
                                                    // neither a string slice '&str'
    _ = output_file
        .write_fmt(format_args!("Final sum result: {}\n", final_result))
        .expect("Failed to write to file!");

    Ok(())
}
