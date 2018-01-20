extern crate csv;
#[macro_use]
extern crate json;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use csv::Reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: &String = &args[1];
    println!("call path is {}", &args[0]);
    println!("file_name is {}", file_name);
    
    let mut file:File = File::open(file_name)
        .expect("File not found");

    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    
    let mut json = object!{};
    
    let mut rdr = Reader::from_reader(contents.as_bytes());
    let headers = rdr.headers()
        .expect("There was an error reading the headers.");
    let mut records_iter = rdr.records();

    loop  {
        match records_iter.next() {
            Some(record) => {
                let record: Vec<String> = record.unwrap();
                println!("Records {:?}", record);

                for index in 0..headers.len() {
                    let key: &str = &record[0];
                    if index == 0 {
                        json[key] = "{}".into();
                    }
                    else {
                        let header: &str = &headers[index][..];
                        let value: &str = &record[index];
                        json[key][header] = value.into();
                    }
                }                
            },
            None => { break }
        }
    }
    


    println!("{}", json.to_string());
    
}
