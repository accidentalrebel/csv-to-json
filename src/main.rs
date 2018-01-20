extern crate csv;
#[macro_use]
extern crate json;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use csv::Reader;
use json::JsonValue;

fn get_file_names(args: Vec<String>) -> (String, String) {
    if args.len() < 2 {
        panic!("Invalid number of arguments!");
    }
    if !args[1].contains(".csv") {
        panic!("scr file is invalid. Should be specified and should contain the .csv extension!");
    }
    if args.len() > 2 && !args[2].contains(".json") {
        panic!("destination file is invalid. Should be specified and should contain the .json extension!");
    } 
    
    let src_file_name: String = args[1].to_owned();
    
    let dest_file_name: String = {
        let splitted: Vec<&str>;
        if args.len() <= 2 {
            splitted = src_file_name.split(".").collect();
            let mut dest_name = splitted[0].to_string();
            dest_name.push_str(".json");
            dest_name.to_owned()
        }
        else {
            args[2].to_owned()
        }
    };

    (src_file_name, dest_file_name)
}

fn update_json_with_record_row<'a>(json: &'a mut JsonValue, record: Vec<String>, headers: &Vec<String>) -> &'a JsonValue {
    let record: Vec<String> = record;

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
    json
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        panic!("Invalid number of arguments!");
    }  
    
    let (src_file_name, dest_file_name) = get_file_names(args);

    println!("src_file_name: {}", src_file_name);
    println!("dest_file_name: {}\n", dest_file_name);
    
    let mut src_file:File = File::open(src_file_name)
        .expect("File not found");

    let mut contents: String = String::new();
    src_file.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");
    
    let mut json: JsonValue = object!{};
    
    let mut rdr: Reader<&[u8]> = Reader::from_reader(contents.as_bytes());
    let headers: Vec<String> = rdr.headers()
        .expect("There was an error reading the headers.");
    
    let mut records_iter = rdr.records();

    loop  {
        match records_iter.next() {
            Some(record) => {
                update_json_with_record_row(&mut json, record.unwrap(), &headers);
            }
            None => { break }
        }
    }

    println!("Converted output:\n{}", json.to_string());
    
    let mut dest_file:File = File::create(&dest_file_name)
        .expect(&format!("Error creating the file: {}", dest_file_name)[..]);
    dest_file.write_all(json.to_string().as_bytes())
        .expect(&format!("Error writing to file: {}", dest_file_name)[..]);

    println!("Successfully wrote to file {}", dest_file_name);
}

#[cfg(test)]
mod tests {
    #[test]
    fn file_names() {
        let (src, dest) = super::get_file_names(vec!["path".to_string(), "csv.csv".to_string(), "csv.json".to_string()]);
        assert_eq!(src, "csv.csv");
        assert_eq!(dest, "csv.json");

        // If no dest file name is specified
        let (src, dest) = super::get_file_names(vec!["path".to_string(), "csv.csv".to_string()]);
        assert_eq!(src, "csv.csv");
        assert_eq!(dest, "csv.json");
    }

    #[test]
    #[should_panic]
    fn panic_file_names() {
        super::get_file_names(vec!["path".to_string()]);
    }

    #[test]
    #[should_panic]
    fn panic_no_extensions() {
        super::get_file_names(vec!["path".to_string(), "csv".to_string(), "csv".to_string()]);
    }

    #[test]
    #[should_panic]
    fn panic_diff_file_names() {
        super::get_file_names(vec!["path".to_string(), "csv.json".to_string(), "csv.csv".to_string()]);
    }
}
