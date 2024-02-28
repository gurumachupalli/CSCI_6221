use csv::ReaderBuilder;
use std::fs::File;

fn convert_to_utf8(strings: &[String]) -> Vec<u8> {
    strings
        .iter()
        .flat_map(|s| s.bytes())
        .collect()
}

fn convert_to_string(utf8_values: &[u8]) -> Vec<String> {
    let string_values: String = utf8_values
        .split(|&byte| byte == b',')
        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
        .collect();

    // Split the string by commas and collect into Vec<String>
    string_values
        .split(',')
        .map(|s| s.trim().to_string())  // Trim to remove leading/trailing whitespaces
        .collect()
}

fn csv_to_matrix(file_path: &str) -> Result<(), csv::Error> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let row_data: Vec<String> = record.iter().map(|field| field.to_string()).collect();
        let utf8_row = convert_to_utf8(&row_data);

        let string_row = convert_to_string(&utf8_row);

        println!("Original Row: {:?}", row_data);
        println!("UTF-8: {:?}", utf8_row);
        println!("Converted Back: {:?}", string_row);
        println!();
    }

    Ok(())
}

fn main() {
    let file_path = "C:\\Users\\17038\\a_Spring_2024\\Paradigms\\encrypt\\data.csv";
    match csv_to_matrix(file_path) {
        Ok(_) => {} // Do nothing if successful
        Err(e) => eprintln!("Error: {}", e),
    }
}
