use std::fs::ReadDir;
use std::fs;
use std::path::Path;

mod scov_tree;
use scov_tree::javascript::js_tokenizer;
use scov_tree::javascript::js_token_parser;

pub fn run(paths: ReadDir, base: String){
    let mut list: Vec<String> = Vec::new();
    
    get_files(paths, &mut list);

    let mut js_files: Vec<String> = Vec::new();
    let mut rust_files: Vec<String> = Vec::new();
    let mut java_files: Vec<String> = Vec::new();
    let mut python_files: Vec<String> = Vec::new();

    for file in list{
        if let Some(ext) = Path::new(&file).extension(){
            match ext.to_str().unwrap(){
                "rs" => rust_files.push(file.clone()),
                "js" => js_files.push(file.clone()),
                "java" => java_files.push(file.clone()),
                "py" => python_files.push(file.clone()),
                _ => (),
            }
        }
    }

    println!("Javascript Files: {:#?}", js_files);
    println!("Rust Files: {:#?}", rust_files);
    println!("Java Files: {:#?}", java_files);
    println!("Python Files: {:#?}", python_files);

    let code = fs::read_to_string(js_files.get(2).unwrap()).expect("error reading file");

    let tokens = js_tokenizer::tokenize(&code);
    let imports = js_token_parser::parse_imports(&tokens);

    for imp in imports{
        println!("{:#?}", imp);
    }
    
}

fn get_files(paths: ReadDir, list: &mut Vec<String>){
    for path in paths{
            let path = path.unwrap().path();
            
            if path.is_dir(){
                get_files(fs::read_dir(path).unwrap(),list);
            }
            else{
                list.push(path.display().to_string());
            }  
    }
}
