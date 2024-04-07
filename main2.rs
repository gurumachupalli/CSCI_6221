use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use nalgebra::DMatrix;


fn initialize_file_reader<P: AsRef<Path>>(path: P) -> io::Result<io::BufReader<File>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file))
}

fn process_file<P: AsRef<Path>>(path: P) -> Result<DMatrix<f64>, Box<dyn std::error::Error>> {
    let reader = initialize_file_reader(&path)?;
    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut numbers = Vec::new();
        let mut current_number = String::new();
        
        for fart in line.chars() {
            if fart.is_digit(10) || fart == '.' || fart == '-' {
                current_number.push(fart);
            } else if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push(num);
                    current_number.clear();
                } else {
                    // Handle parsing error here
                }
            }
        }
        if !numbers.is_empty() {
            matrix.push(numbers);
        }

        // for row in &matrix {
        //     println!("{:?}", row);
        // }
    }
    // Convert pushed numbers array to matrix<f64>
    // Convert Vec<Vec<u64>> to nalgebra DMatrix
    let rows = matrix.len();

    let cols = matrix[0].len(); // Assuming all inner vectors have the same length
    let output_matrix = DMatrix::from_fn(rows, cols, |i, j| matrix[i][j]);

    Ok(output_matrix)
}

fn decrypt(encrypted_matrix: Result<DMatrix<f64>, Box<dyn std::error::Error>>, secret_key: Result<DMatrix<f64>, Box<dyn std::error::Error>>) -> Vec<Vec<u64>> {
    // Unwrap the results or handle the errors
    let encrypted_matrix = match encrypted_matrix {
        Ok(matrix) => matrix,
        Err(_err) => return vec![vec![]], // Return an empty vector if there's an error
    };
    let secret_key = match secret_key {
        Ok(matrix) => matrix,
        Err(_err) => return vec![vec![]], // Return an empty vector if there's an error
    };

    // Double check encryption isn't invalid
    if encrypted_matrix.ncols() != secret_key.nrows() {
        println!("Incompatible dimensions for matrix multiplication");
        return vec![vec![]]; // Return an empty vector if dimensions are incompatible
    }

    // Perform matrix multiplication
    let testing_f64 = encrypted_matrix * secret_key;

    // Convert testing matrix back to u64 and maintain shape
    let mut decrypted_matrix = Vec::new();
    for row in testing_f64.row_iter() {
        let mut row_values = Vec::new();
        for &elem in row.iter() {
            row_values.push(elem.round() as u64);
        }
        decrypted_matrix.push(row_values);
    }

    decrypted_matrix
}

fn utf8_to_string(strings: Vec<Vec<u64>>) -> Result<(), Box<dyn Error>> {
    for line in strings {
        // Convert UTF-8 values back to bytes
        let mut bytes = Vec::new();

        for &val in &line {
            bytes.push(val as u8);
        }

        // Convert bytes to string
        if let Ok(string) = String::from_utf8(bytes) {
            println!("{}", string);
        } else {
            return Err("Failed to convert bytes to string".into());
        }
    }

    Ok(())
}


fn main() -> io::Result<()> {
    // Specify the path to the text file
    let file_path = "C:\\Users\\17038\\a_Spring_2024\\Paradigms\\Encrypt_testing\\decrypt\\data\\encrypted_matrix.txt";
    // Process the file
    let encrypted_matrix = process_file(file_path);
    let file_path = "C:\\Users\\17038\\a_Spring_2024\\Paradigms\\Encrypt_testing\\decrypt\\data\\secret_key.txt";
    let secret_key = process_file(file_path);

    let result = decrypt(encrypted_matrix, secret_key);
    // Print the shape of the decrypted matrix
    let num_rows = result.len();
    let num_cols = if let Some(row) = result.get(0) {
        row.len()
    } else {
        0
    };
    println!("Number of rows in decrypted matrix: {}", num_rows);
    println!("Number of columns in decrypted matrix: {}", num_cols);
    utf8_to_string(result);

    Ok(())
}
