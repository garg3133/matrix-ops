use rand::Rng;
use std::{env, fs};
use std::io::{self, Write};

const DEFAULT_MATRIX_FILENAME: &str = "matrix.txt";

fn get_args() -> (String, u32) {
    let mut argv_iter = env::args();

    // ignore first argv (path to binary)
    argv_iter.next();

    let mut file_name: Option<String> = None;
    let mut size: Option<String> = None;

    // TODO: Improve error handling
    loop {
        let next_argv = argv_iter.next().unwrap_or("".to_string());
        if next_argv == "" {
            break;
        }
        
        if next_argv == "--out" {
            if let Some(_) = file_name {
                // file_name already set
                panic!("--out flag can only be passed once.");
            }

            let flag_value = argv_iter.next();

            if let Some(value) = flag_value {
                file_name = Some(value);
            } else {
                println!("No argument passed for --out, defaulting to: '{DEFAULT_MATRIX_FILENAME}'");
                break;
            }
        } else {
            if let Some(_) = size {
                // size already set
                panic!("Too many or wrong arguments passed");
            } else {
                size = Some(next_argv);
            }
        }
    }

    let file_name = if let Some(value) = file_name {
        value
    } else {
        DEFAULT_MATRIX_FILENAME.to_string()
    };

    let size: String = if let Some(value) = size {
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

