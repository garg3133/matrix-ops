use rand::Rng;
use std::{env, fs};
use std::io::{self, Write};

const DEFAULT_MATRIX_FILENAME: &str = "matrix.txt";

fn get_args() -> (String, u32, u32) {
    let mut argv_iter = env::args();

    // ignore first argv (path to binary)
    argv_iter.next();

    let mut file_name: Option<String> = None;
    let mut row_size: Option<String> = None;
    let mut col_size: Option<String> = None;

    // TODO: Improve error handling (to panic! or not to panic!)
    loop {
        let next_argv = argv_iter.next().unwrap_or("".to_string());
        if next_argv == "" {
            break;
        }
        
        if next_argv == "--out" {
            if let Some(_) = file_name {
                // file_name already set
                help();
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
            if let Some(_) = row_size {
                if let Some(_) = col_size {
                    // row_size and col_size already set
                    help();
                    panic!("Too many or wrong arguments passed");
                } else {
                    col_size = Some(next_argv);
                }
            } else {
                row_size = Some(next_argv);
            }
        }
    }

    let file_name = if let Some(value) = file_name {
        value
    } else {
        DEFAULT_MATRIX_FILENAME.to_string()
    };

    let (row_size, col_size): (String, String) = if let Some(value1) = row_size {
        if let Some(value2) = col_size {
            (value1, value2)
        } else {
            (value1.clone(), value1)
        }
    } else {
        // get row size from user
        let mut row_size = String::new();
        print!("Expected no. of rows in matrix: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut row_size)
            .expect("Failed to read input");

        // get column size from user
        let mut col_size = String::new();
        print!("Expected no. of columns in matrix: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut col_size)
            .expect("Failed to read input");

        (row_size, col_size)
    };

    let row_size: u32 = row_size.trim().parse().expect("Matrix row size should be a number.");
    let col_size: u32 = col_size.trim().parse().expect("Matrix col size should be a number.");

    (file_name, row_size, col_size)
}

fn main() {
    let (file_name, row_size, col_size) = get_args();

    // random number generator (for matrix elements)
    let mut thread_rng = rand::thread_rng();

    let mut final_matrix = String::new();

    (0..row_size).for_each(|_| {
        (0..col_size).for_each(|_| {
            let num = thread_rng.gen_range(1..100);
            final_matrix.push_str(&format!("{num:0>2} ")); // https://doc.rust-lang.org/rust-by-example/hello/print.html
        });
        final_matrix.push('\n');
    });

    if let Err(error) = fs::write(&file_name, final_matrix.as_bytes()) {
        panic!("Error while creating or writing to file '{file_name}': {error}");
    }

    println!("Created a matrix of size {row_size}x{col_size} in '{file_name}'!");
}

fn help() {
    println!("USAGE:");
    println!("  cargo run --bin create [<row_size>] [<col_size>] [--out <file_name>]\n");
}

