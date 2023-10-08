extern crate serde;
extern crate csv;



use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use csv::WriterBuilder;

#[derive(Debug, Serialize, Deserialize)]
struct MyObject {
    // Define your struct fields here
    title1: String,
    title2: String,
    // Add more fields as needed
}

fn main() -> Result<(), Box<dyn Error>> {

    let input_file = File::open("path/to/input.txt")?;

    let objects: Vec<MyObject> = serde_json::from_reader(input_file)?;

    for obj in &objects {
        println!("{:?}", obj);
    }
    

    let mut csv_writer = WriterBuilder::new().from_path("../output.csv")?;

    //  Write the CSV header
    csv_writer.write_record(&["title1", "title2"])?;

    // Write each object to the CSV file
    for obj in &objects {
        csv_writer.serialize(obj)?;
    }

    csv_writer.flush()?;

    Ok(())
}
