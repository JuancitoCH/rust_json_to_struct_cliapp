use serde_json::{Value,Map};
use chrono::Local;

pub fn escribir_file(json_content:String,output_path:String){
    std::fs::write(
        output_path,
        json_content
    )
    .unwrap();
}

pub fn construir_struct(json_object:&Map<String,Value>,name:Option<String>)->String{
    let struct_name = match name{
        Some(re)=>re,
        None=>String::from("generated")
    };
    let mut response = format!("pub struct {} {{\n",struct_name);
    generate_types_for_object(json_object,&mut response);
    response.push_str("}");
    response
}

pub fn validate_type(value:&Value, aux_for_object:&mut String)->String{
    if value.is_string(){
        String::from("String")
    }else if value.is_array(){
        format!("Vec<{}>" , validate_type(&value.as_array().unwrap()[0],aux_for_object) )
    }else if value.is_boolean(){
        String::from("bool")
    }else if value.is_f64(){
        String::from("f64")
    }else if value.is_i64(){
        String::from("i64")
    }else if value.is_u64(){
       String::from( "u64")
    }else if value.is_object(){
        // println!("{:?}",time::SystemTime::now() as String);
        let unique_name = format!("Objeto_Capa{}",Local::now());
        let unique_name=unique_name.replace("-","");
        let unique_name=unique_name.replace(":","");
        let unique_name=unique_name.replace(" ","");
        let unique_name=unique_name.replace(".","");
        let text:String = value.to_string();
        let json_object = serde_json::from_str::<Value>(&text).unwrap();
        let json_object = json_object.as_object().unwrap();

        let response_1 = &construir_struct(json_object,Option::Some(unique_name.clone()) )[..];
        aux_for_object.push_str(response_1);
        unique_name
    }else{
        String::from("String")
    }
}

pub fn generate_types_for_object(json_object:&Map<String,Value>,response:&mut String){
    let mut aux_for_object = String::new();
    for (keys,value) in json_object.iter(){
        response.push_str("\t");
        response.push_str(keys);
        response.push_str(":");
        response.push_str(&validate_type(&value,&mut aux_for_object)[..]);
        response.push_str(",");
        response.push_str("\n");
    }
    aux_for_object.push_str("\n");
    aux_for_object.push_str(response);
    response.clear();
    response.push_str(&aux_for_object)
}

