use std::env;
use std::fs;
use std::io;
use std::path;
use std::process;
use std::thread;

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
    let mut children = vec![];

    /*************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     ************************************************************************/

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

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately
    // "destructured" into two variables, "i" and "data_segment" with a
    // "destructuring assignment"

    // for (i, data_segment) in chunked_data.enumerate() {
    // NOTE: (aver) As suggested by ChatGPT:
    // By using into_iter() instead of iter(), we consume the vector and transfer ownership of each
    // String segment to the closure. This way, the closure can access each String for as long as
    // it needs to without worrying about the data being dropped prematurely.
    for (i, data_segment) in chunked_data.into_iter().enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.

        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                // iterate over the characters of our segment..
                .chars()
                // .. convert text-characters to their number value..
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // .. and sum the resulting iterator of numbers
                .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result
        }));
    }

    /*************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     ************************************************************************/

    // combine each thread's intermediate results into a single final sum.
    //
    // we use the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
    Ok(())
}
