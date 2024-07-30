# matrix-ops

## Usage

### 1. Create a square matrix

**Usage:** `cargo run --bin create [<matrix-size>] [--out <file-name>]`

```sh
# output a square matrix to 'matrix.txt' file.
# program will ask for matrix size.
cargo run --bin create

# output a square matrix of size 8 to 'matrix.txt' file.
cargo run --bin create 8

# output a square matrix of size 10 to 'new_matrix.txt' file.
cargo run --bin create 10 --out new_matrix.txt

# output a square matrix to 'matrix_2.txt' file.
# program will ask for matrix size.
#
# if the first argument is a flag, it should be preceded by a `--`.
cargo run --bin create -- --out matrix_2.txt
```

### 2. Calculate determinant of a square matrix

**Usage:** `cargo run --bin determinant <matrix-file-path>`

```sh
# calculate determinant of the square matrix
# present in file `matrix1.txt`.
cargo run --bin determinant matrix1.txt

# calculate determinant of the square matrix
# present in file `matrix.txt` (default).
cargo run --bin determinant
```

