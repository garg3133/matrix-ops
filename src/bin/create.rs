use rand::Rng;
use std::{env, fs};
use std::io::{self, Write};

fn get_args() -> (String, u32) {
    let mut args_iter = env::args();
    // first item not useful to us
    args_iter.next();

    let file_name = if let Some(value) = args_iter.next() {
        value
    } else {
        "matrix.txt".to_string()
    };

    let size: String = if let Some(value) = args_iter.next() {
        value
    } else {
        let mut input = String::new();

        print!("Enter matrix size: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input
    };
    let size: u32 = size.trim().parse().expect("Matrix size should be a number.");

    (file_name, size)
}

fn main() {
    let (file_name, size) = get_args();

    // random number generator (for matrix elements)
    let mut thread_rng = rand::thread_rng();

    let mut final_matrix = String::new();

    (0..size).for_each(|_| {
        (0..size).for_each(|_| {
            let num = thread_rng.gen_range(1..100);
            final_matrix.push_str(&format!("{num:0>2} ")); // https://doc.rust-lang.org/rust-by-example/hello/print.html
        });
        final_matrix.push('\n');
    });

    if let Err(error) = fs::write(&file_name, final_matrix.as_bytes()) {
        panic!("Error while creating or writing to file '{file_name}': {error}");
    }

    println!("Created a matrix of size {size} in '{file_name}'!");
}

