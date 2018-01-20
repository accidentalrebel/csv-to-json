extern crate csv;
#[macro_use]
extern crate json;

use csv::Reader;

fn main() {
    let csv_string: &str = r#"
textID,english,tagalog
GUESS_THE_INGREDIENT,Guess the Ingredient,Hulaan ang Ingredient
GUESS_THE_DISH,Guess the Dish,Hulaan ang luto
"#;

    /*
{
    "GUESS_THE_INGREDIENT" : {
        "english": "Guess the Ingredient",
        "tagalog": "Hulaan ang Ingredient"
    },
    "GUESS_THE_DISH": {
        "english": "Guess the Dish",
        "tagalog": "Hulaan ang luto"
    }
}
     */

    let mut json = object!{};
    
    let mut rdr = Reader::from_reader(csv_string.as_bytes());
    let headers = rdr.headers().unwrap();
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
