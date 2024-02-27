use csv::ReaderBuilder;
use std::fs::File;

fn csv_to_matrix(file_path: &str) -> Result<Vec<Vec<String>>, csv::Error> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut matrix: Vec<Vec<String>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let row_data: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        matrix.push(row_data);
    }

    Ok(matrix)
}

fn main() {
    let file_path = "C:\\Users\\17038\\a_Spring_2024\\Paradigms\\encrypt\\data.csv";
    match csv_to_matrix(file_path) {
        Ok(matrix) => {
            // Print the resulting matrix for demonstration
            for row in &matrix {
                println!("{:?}", row);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
