use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    let mut size = String::new();

    let mut args_iter = env::args();
    // first item not useful to us
    args_iter.next();

    // define size of matrix
    if let Some(value) = args_iter.next() {
        size = value;
    } else {
        print!("Enter matrix size: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut size)
            .expect("Failed to read input");
    }

    let size: u32 = size.trim().parse().expect("Matrix size should be a number.");

    // define file in which matrix should be saved
    let file_name = if let Some(value) = args_iter.next() {
        value
    } else {
        "matrix.txt".to_string()
    };
    let mut file = File::create(&file_name).expect("Failed to create file");

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

    file.write(final_matrix.as_bytes()).expect("Failed to write matrix into file");

    println!("Created a matrix of size {size} in '{file_name}'!");
}
