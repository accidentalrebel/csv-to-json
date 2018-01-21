extern crate csv;
#[macro_use]
extern crate json;
extern crate getopts;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use csv::Reader;
use json::JsonValue;
use getopts::Options;
use getopts::Matches;

fn get_file_names(input: String, output: Option<String>) -> (String, String) {
    if !input.contains(".csv") {
        panic!("src file is invalid. Should be specified and should contain the .csv extension!");
    }

    let src_file_name: String = input;
    let dest_file_name: String = {
        match output {
            Some(output_string) => {
                if !output_string.contains(".json") {
                    panic!("destination file is invalid. Should be specified and should contain the .json extension!");
                }
                output_string
            }
            None => {
                let splitted: Vec<&str> = src_file_name.split('.').collect();
                let mut dest_name = splitted[0].to_string();
                dest_name.push_str(".json");
                dest_name.to_owned()
            }
        }
    };

    (src_file_name, dest_file_name)
}

struct Args {
    input: String,
    output: Option<String>,
}

fn get_args(arg_strings: Vec<String>) -> Option<Args> {
    println!("ARGS: {:?}", arg_strings);
    let mut opts: Options = Options::new();
    opts.optopt("o", "", "The path of the output file including the file extension.", "FILE");
    opts.optflag("h", "help", "Prints this help menu.");
    let matches: Matches = match opts.parse(&arg_strings[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let program: String = arg_strings[0].clone();
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return None;
    }
    
    let output: Option<String> = matches.opt_str("o");
    let input: String = if !matches.free.is_empty() {
        matches.free[0].clone()
    }
    else {
        print_usage(&program, &opts);
        return None;
    };
    Some(Args {input, output})
}

fn update_json_with_record_row(mut json: JsonValue, record: Vec<String>, headers: &[String]) -> JsonValue {
    let record: Vec<String> = record;

    for index in 0..headers.len() {
        if index >= record.len() {
            break;
        }
        
        let key: &str = &record[0];
        if index == 0 {
            json[key] = object!{};
        }
        else {
            let header: &str = &headers[index][..];
            let value: &str = &record[index];
            json[key][header] = value.into();
        }
    }
    json
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let arg_strings: Vec<String> = env::args().collect(); 
    let args: Args = match get_args(arg_strings) {
        Some(args) => { args }
        None => {
            return;
        }
    };
        
    let (src_file_name, dest_file_name) = get_file_names(args.input, args.output);

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

    while let Some(record) = records_iter.next() {
        json = update_json_with_record_row(json, record.unwrap(), &headers);
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
    fn updating_json() {
        let mut json: super::JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        let headers: Vec<String> = vec![String::from("header_a"), String::from("header_b"), String::from("header_c")];
        json = super::update_json_with_record_row(json, record, &headers);
        assert_eq!(json.to_string(), object!{
            "a" => object!{
                "header_b" => "b",
                "header_c" => "c"
            }
        }.to_string());

        // If there is less column on the record
        let mut json: super::JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        let headers: Vec<String> = vec![String::from("header_a"), String::from("header_b")];
        json = super::update_json_with_record_row(json, record, &headers);
        assert_eq!(json.to_string(), object!{
            "a" => object!{
                "header_b" => "b"
            }
        }.to_string());

        // If there is one column on the record.
        let mut json: super::JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b"), String::from("c")];
        let headers: Vec<String> = vec![String::from("header_a")];
        json = super::update_json_with_record_row(json, record, &headers);
        assert_eq!(json.to_string(), object!{
            "a" => object!{
            }
        }.to_string());

        // If there are more record columns than headers
        let mut json: super::JsonValue = object!{};
        let record: Vec<String> = vec![String::from("a"), String::from("b")];
        let headers: Vec<String> = vec![String::from("header_a"), String::from("header_b"), String::from("header_c")];
        json = super::update_json_with_record_row(json, record, &headers);
        assert_eq!(json.to_string(), object!{
            "a" => object!{
                "header_b" => "b"
            }
        }.to_string());
    }
    
    #[test]
    fn file_names() {
        let (src, dest) = super::get_file_names(String::from("csv.csv"), Some(String::from("csv.json")));
        assert_eq!(src, "csv.csv");
        assert_eq!(dest, "csv.json");

        // If no dest file name is specified
        let (src, dest) = super::get_file_names(String::from("csv.csv"), None);
        assert_eq!(src, "csv.csv");
        assert_eq!(dest, "csv.json");
    }

    #[test]
    #[should_panic]
    fn panic_no_extensions() {
        super::get_file_names(String::from("csv"), Some(String::from("csv")));
    }

    #[test]
    #[should_panic]
    fn panic_diff_file_names() {
        super::get_file_names(String::from("csv.json"), Some(String::from("csv.csv")));
    }
}
