use serde_json::{Value};
use json_to_struct::features::{escribir_file,construir_struct};
fn main() {
    // Get the filenames from the command line.
    let input_path = std::env::args().nth(1).unwrap_or(String::from("input.json"));
    // let output_path = std::env::args().nth(2).unwrap_or(String::from("output.json"));

    let json_file = {
        // Load the first file into a string.
        let text = match std::fs::read_to_string(&input_path){
            Ok(res)=>res,
            Err(_)=>{
                // si no encuentra el archivo lo creamos
                std::fs::File::create(&input_path).unwrap();
                std::fs::write(&input_path, "{}").unwrap();
                std::fs::read_to_string(&input_path).unwrap()
            }
        };
        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    let json_object = json_file.as_object().unwrap();
    let struct_generate_text = construir_struct(json_object,Option::None);
    println!("{}",struct_generate_text);
    escribir_file(struct_generate_text,String::from("./struct_generated.rs"));
}


