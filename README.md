# CSV to JSON
A simple CSV to JSON command line app written in Rust.

## Usage
```
Usage: csv_to_json SOURCE_FILE_NAME [options]

Options:
    -o TARGET_FILE_NAME The path of the output file including the file
                        extension.
    -n, --null          Empty strings are set to null.
    -k, --keyed         Generate output as keyed JSON.
    -h, --help          Prints this help menu.
````

## Installation / Compilation

I haven't made any binaries yet but you can easily compile it if you have _Rust_ and _Cargo_ installed. Pull the repository and call `cargo run --release` to compile.

## Examples
The example below would parse the data inside sample.csv, convert to json, and write to output.json. The output file is created if it doesn't exist. Otherwise, the contents will be overwritten.
```
csv_to_json sample.csv -o output.json
```

If no output file is specified then the name of the source file will be used and will append ".json" to it. In the example below, sample.csv will generate sample.json.

```
csv_to_json sample.csv
```

## Options

### -n, --null
```
-n, --null          Empty strings are set to null.
```
If the above option is specified then all empty data would have a `null` value as shown below.

```
textID,english,tagalog
GUESS_THE_INGREDIENT,Guess the Ingredient,Hulaan ang Ingredient
GUESS_THE_CHEF,,
```

Would convert to:

```json
[
    {
        "textID":"GUESS_THE_INGREDIENT",
        "english":"Guess the Ingredient",
        "tagalog":"Hulaan ang Ingredient"
    },
    {
        "textID":"GUESS_THE_CHEF",
        "english":null,
        "tagalog":null
    }
]
```

If `-n` is not specified then it will show a `""` instead of `null` as shown below:


```json
[
    {
        "textID":"GUESS_THE_INGREDIENT",
        "english":"Guess the Ingredient",
        "tagalog":"Hulaan ang Ingredient"
    },
    {
        "textID":"GUESS_THE_CHEF",
        "english":"",
        "tagalog":""
    }
]
```

### -k, --k
```
-k, --keyed         Generate output as keyed JSON.
```
If the above option is set then the json structure would be a Keyed JSON structure. The example below shows what it looks like:

```
textID,english,tagalog
GUESS_THE_INGREDIENT,Guess the Ingredient,Hulaan ang Ingredient
GUESS_THE_CHEF,,
GUESS_THE_DISH,Guess the Dish,Hulaan ang luto
```

```
{
    "GUESS_THE_INGREDIENT":{
        "english":"Guess the Ingredient",
        "tagalog":"Hulaan ang Ingredient"
    },
    "GUESS_THE_CHEF":{
        "english":"",
        "tagalog":""
    },
    "GUESS_THE_DISH":{
        "english":"Guess the Dish",
        "tagalog":"Hulaan ang luto"
    }
}
```
Compare the above with the a non-keyed JSON structure as shown below:
```
[
    {
        "textID":"GUESS_THE_INGREDIENT",
        "english":"Guess the Ingredient",
        "tagalog":"Hulaan ang Ingredient"
    },
    {
        "textID":"GUESS_THE_CHEF",
        "english":"",
        "tagalog":""
    },
    {
        "textID":"GUESS_THE_DISH",
        "english":"Guess the Dish",
        "tagalog":"Hulaan ang luto"
    }
]
```

## Contributing

All contributions are welcome!
