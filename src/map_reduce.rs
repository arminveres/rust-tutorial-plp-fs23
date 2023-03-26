use std::thread;
use std::thread::JoinHandle;

pub fn map(chunked_data:Vec<String>) -> Vec<JoinHandle<u32>> {

    let mut children = vec![];

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

    return children;
}

pub fn reduce(children_vector: Vec<JoinHandle<u32>>) -> u32 {
    let final_result = children_vector.into_iter().map(|c| c.join().unwrap()).sum::<u32>();
    return final_result;
}

