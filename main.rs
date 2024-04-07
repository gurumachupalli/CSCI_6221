use std::error::Error; //Useful for debugging
use csv::ReaderBuilder; //Reading input file
use nalgebra::DMatrix;
use approx::relative_eq;
use std::fs::File;
use std::io::prelude::*;
// ENCRYPTION PROCESS
// Function for determining longest row after utf-8 to perform padding for matrix operations
fn find_max_row_length(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?; // Initialize a CSV reader
    let headers = rdr.headers()?.clone(); // Clone the headers for manipulation
    let mut max_row_length = string_array_to_utf8_array(&headers)?; // Convert headers to UTF-8 and get starting max length

    for result in rdr.records() { // Iterate through rows of csv file
        let record = result?; // Assign line to record
        let row_length = string_array_to_utf8_array(&record)?; // Check row length
        if row_length > max_row_length {
            max_row_length = row_length; // Replace if larger than max row length
        }
    }
    Ok(max_row_length)
}

// Function for converting string read from csv into utf8 ndarray to determine longest line in csv file
fn string_array_to_utf8_array(strings: &csv::StringRecord) ->  Result<usize, Box<dyn Error>>  {
    let mut utf8_values: Vec<u64> = Vec::new(); //Initialize utf8 values
    for (i, s) in strings.iter().enumerate() { //Iteration through line
        utf8_values.extend(s.bytes().map(|b| <u8 as Into<u64>>::into(b)));
        if i < strings.len() - 1 {
            utf8_values.push(b',' as u64); //Ensure correct conversion i.e adding with commas for correct formatting of conversion
        }
    }
    // Create a DMatrix with one row and a dynamic number of columns
    let num_cols = utf8_values.len();

    Ok(num_cols) //take clones values to determine
}

fn fart(file_path: &str, max_row_length: usize) -> Result<DMatrix<u64>, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let headers = rdr.headers()?.clone(); // Clone the headers

    // Initialize matrix with header as first row
    let mut rows = vec![utf8_array_pad_to_max_length(&headers, max_row_length)?];

    for result in rdr.records() {
        let record = result?;
        let row_array = utf8_array_pad_to_max_length(&record, max_row_length)?;
        rows.push(row_array);
    }

    // Convert Vec<Vec<u64>> to nalgebra DMatrix
    let num_cols = rows[0].ncols();
    let num_rows = rows.len();
    let mut matrix_data = Vec::new();
    for row in &rows {
        matrix_data.extend_from_slice(row.as_slice());
    }
    let matrix = DMatrix::from_row_slice(num_rows as usize, num_cols, &matrix_data);

    Ok(matrix)
}

fn utf8_array_pad_to_max_length(strings: &csv::StringRecord, max_length: usize) -> Result<DMatrix<u64>, Box<dyn Error>> {
    let mut utf8_values: Vec<u64> = Vec::new();
    for (i, s) in strings.iter().enumerate() {
        utf8_values.extend(s.bytes().map(|b| <u8 as Into<u64>>::into(b)));
        if i < strings.len() - 1 {
            utf8_values.push(b',' as u64);
        }
    }
    // Pad with ASCII 124 (|) Change to pad with ASCII 124 and ASCII 126
    while utf8_values.len() < max_length {
        utf8_values.push(124);
    }
    // Create a DMatrix with one row and a dynamic number of columns
    let num_cols = utf8_values.len();
    let matrix = DMatrix::<u64>::from_row_slice(1, num_cols, utf8_values.as_slice());

    Ok(matrix)
}

fn generate_encrypted_matrix(original_matrix: &DMatrix<u64>) -> Option<(DMatrix<f64>, DMatrix<f64>, f64)> {
    let size = original_matrix.ncols(); // Get the number of columns in the original matrix

    // Create a secret key matrix with the same size as the original matrix
    let mut encrypt_matrix = DMatrix::<f64>::zeros(size, size);
    // Initialize public key, will be used to hold determinant

    // Populate the matrix with random ones
    for i in 0..size {
        for j in 0..size {
            // Generate a random number between 0 and 9
            let random_number = rand::random::<u8>() % 10;
            // If the random number is less than 5, set the element to 1
            if random_number < 5 {
                encrypt_matrix[(i, j)] = 1.0;
            }
        }
    }
    
    // Check if the determinant of the secret key is zero
    if encrypt_matrix.determinant().abs() < f64::EPSILON {
        // If the determinant is close to zero, recursively call the function
        return generate_encrypted_matrix(original_matrix);
    }

    // Compute the inverse
    let secret_key = encrypt_matrix.clone().try_inverse()?;

    // Convert the original matrix to f64
    let original_matrix_f64 = original_matrix.map(|elem| elem as f64);


    // Matrix multiplication to get the encrypted matrix
    let encrypted_matrix = original_matrix_f64.clone() * encrypt_matrix;

    // Solve for the public key
    let public_key = secret_key.norm();
    
    // Double check encryption isnt doo doo
    let testing_f64 = encrypted_matrix.clone()* secret_key.clone();
    // let testing = original_matrix_f64.clone();

    // Convert testing matrix back to u64
    let testing_u64 = testing_f64.iter().map(|&elem| elem.round() as u64).collect::<Vec<u64>>();
    // Assuming original_matrix is a DMatrix<u64>
    // and testing_u64 is also a DMatrix<u64>
    // Define matrices

    // Check if matrices are approximately equal
    let are_matrices_equal = relative_eq!(&original_matrix_f64, &testing_f64, epsilon = 1.0e-6, max_relative = 1.0e-4);

    if are_matrices_equal {
        println!("Matrices are approximately equal.");
    } else {
        println!("Matrices are not approximately equal.");
    }

    // comparing u64
    let are_matrices_equal = original_matrix.iter().zip(testing_u64.iter()).all(|(&orig_elem, &test_elem)| orig_elem == test_elem);
    if are_matrices_equal {
        println!("testing_u64 is the same as original_matrix.");
    } else {
        println!("testing_u64 is not the same as original_matrix.");
    }

    let size = encrypted_matrix.ncols();
    println!("SHITBRODDA {}",size);

    Some((encrypted_matrix, secret_key, public_key))
}

// Save encryption process to respective files
fn save_matrices_to_files(encrypted_matrix: &nalgebra::DMatrix<f64>,secret_key: &nalgebra::DMatrix<f64>,public_key: f64,) -> Result<(), Box<dyn Error>> {
    // Save encrypted matrix to file
    let mut encrypted_matrix_file = File::create("encrypted_matrix.txt")?;
    encrypted_matrix_file.write_all(format!("{}", encrypted_matrix).as_bytes())?;

    // Save secret key to file
    let mut secret_key_file = File::create("secret_key.txt")?;
    secret_key_file.write_all(format!("{}", secret_key).as_bytes())?;

    // Save public key to file
    let mut public_key_file = File::create("public_key.txt")?;
    public_key_file.write_all(format!("{}", public_key).as_bytes())?;

    // Ignore the unused Result values
    let _ = encrypted_matrix_file;
    let _ = secret_key_file;
    let _ = public_key_file;

    Ok(())
}

fn main() {
    let file_path = "C:\\Users\\17038\\a_Spring_2024\\Paradigms\\Encrypt_testing\\lattice\\data\\data.csv";
    // Call find_max_row_length to get the maximum row length
    match find_max_row_length(file_path) {
        Ok(max_row_length) => {
            // Call fart with the maximum row length
            match fart(file_path, max_row_length) {
                Ok(matrix) => {
                    // Successfully obtained the matrix
                    println!("Matrix:");
                    // Print each row of the matrix
                    // Print each row of the matrix
                    for i in 0..matrix.nrows() {
                        // Print the row elements
                        println!("{:?}", matrix.row(i).iter().cloned().collect::<Vec<_>>());
                    }
                    // Call generate_encrypted_matrix with the fart output
                    match generate_encrypted_matrix(&matrix) {
                        Some((encrypted_matrix, secret_key, public_key)) => {
                            // Successfully processed and encrypted
                            println!("Successfully processed and encrypted CSV file.");
                            // Now you can work with the encrypted data
                            // Save matrices and key to files
                            match save_matrices_to_files(&encrypted_matrix, &secret_key, public_key) {
                                Ok(()) => {
                                    println!("Matrices and key saved successfully.");
                                }
                                Err(e) => {
                                    eprintln!("Error saving matrices and key: {}", e);
                                }
                            }
                        }
                        None => println!("Failed to generate encrypted matrix."),
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

}

