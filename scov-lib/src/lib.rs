use std::fs::ReadDir;
use std::fs;
use std::path::Path;

mod scov_tree;
use scov_tree::javascript::js_tokenizer;
use scov_tree::javascript::js_token_parser;
use scov_tree::javascript::js_scanner;

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

    let js_list = js_scanner::get_nodes(&js_files);
    for node in js_list{
        println!("{}", node.file_name);
        println!("{}", node.file_path);
        for import in node.imports{
            println!("  Import: {}", import.import.source);
            println!("    Node: {}\n    {}", import.node.as_ref().map_or("None", |n| n.file_name.as_str()), import.node.as_ref().map_or("None", |n| n.file_path.as_str()));
        }
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
