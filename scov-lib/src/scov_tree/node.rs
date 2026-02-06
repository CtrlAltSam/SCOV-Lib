pub struct Node {
    pub file_path: String,
    pub file_name: String,
    pub var: Vec<Variable>,
    pub func: Vec<Function>,

}

pub struct Variable {
    pub name: String,
    pub var_type: String,
    pub node: Node,
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub return_type: Option<String>,
    pub node: Node,
}