use std::fs;
use std::path::{Path, PathBuf};
use super::super::node::Node;
use super::super::node::Imp;
use super::js_tokenizer::tokenize;
use super::js_token_parser::parse_imports;

pub fn get_nodes(files: &Vec<String>) -> Vec<Node> {
    let mut nodes = Vec::new();

    for file in files {
        let code = fs::read_to_string(file).expect("error reading file");

        let tokens = tokenize(&code);
        let imports = parse_imports(&tokens);

        let node = Node {
            file_path: file.clone(),
            file_name: std::path::Path::new(file)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            imports: imports.into_iter().map(|import| Imp {
                import,
                node: None,
            }).collect(),
        };

        nodes.push(node);
    }

    resolve_imports(&mut nodes);

    nodes
}


//ai generated code, want to change later to be more concise.
fn resolve_imports(nodes: &mut Vec<Node>) {
    // Create a map of file paths to node indices for efficient lookup
    let mut path_to_index: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    // Normalize all paths and create the map
    for (idx, node) in nodes.iter().enumerate() {
        if let Ok(canonical) = std::fs::canonicalize(&node.file_path) {
            path_to_index.insert(canonical.to_string_lossy().to_string(), idx);
        }
    }
    
    // Clone the nodes vector to avoid borrow checker issues
    let nodes_clone = nodes.clone();
    
    // Resolve imports for each node
    for node in nodes.iter_mut() {
        let base_path = Path::new(&node.file_path);
        let base_dir = base_path.parent().unwrap_or(Path::new("."));
        
        for imp in node.imports.iter_mut() {
            let import_source = &imp.import.source;
            
            // Resolve the relative path
            let resolved_path = base_dir.join(import_source);
            
            // Try to canonicalize the path
            let canonical_path = if let Ok(canon) = std::fs::canonicalize(&resolved_path) {
                canon.to_string_lossy().to_string()
            } else {
                // If canonicalize fails, try adding .js extension
                let with_js = if !import_source.ends_with(".js") {
                    base_dir.join(format!("{}.js", import_source))
                } else {
                    resolved_path.clone()
                };
                
                if let Ok(canon) = std::fs::canonicalize(&with_js) {
                    canon.to_string_lossy().to_string()
                } else {
                    // Fall back to normalized path
                    resolved_path.to_string_lossy().to_string()
                }
            };
            
            // Find the corresponding node
            if let Some(&target_idx) = path_to_index.get(&canonical_path) {
                imp.node = Some(nodes_clone[target_idx].clone());
            }
        }
    }
}