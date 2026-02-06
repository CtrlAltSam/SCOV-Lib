use super::js_token::JsToken as Token;

#[derive(Debug)]
pub struct Import {
    pub source: String,
    pub default: Option<String>,
    pub named: Vec<String>,
    pub is_dynamic: bool,
}

pub fn parse_imports(tokens: &[Token]) -> Vec<Import> {
    let mut imports = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            // ---------- static import ----------
            Token::Import => {
                if let Some((imp, next)) = parse_static_import(tokens, i) {
                    imports.push(imp);
                    i = next;
                } else {
                    i += 1;
                }
            }

            // ---------- dynamic import ----------
            Token::Ident(name) if name == "import" => {
                if let Some((imp, next)) = parse_dynamic_import(tokens, i) {
                    imports.push(imp);
                    i = next;
                } else {
                    i += 1;
                }
            }

            _ => i += 1,
        }
    }

    imports
}

fn parse_static_import(tokens: &[Token], start: usize) -> Option<(Import, usize)> {
    let mut i = start + 1;
    let mut default = None;
    let mut named = Vec::new();

    // import "mod";
    if let Some(Token::String(src)) = tokens.get(i) {
        return Some((
            Import {
                source: src.clone(),
                default: None,
                named,
                is_dynamic: false,
            },
            i + 1,
        ));
    }

    // import foo ...
    if let Some(Token::Ident(name)) = tokens.get(i) {
        default = Some(name.clone());
        i += 1;
    }

    // import foo, { ... }
    if matches!(tokens.get(i), Some(Token::Comma)) {
        i += 1;
    }

    // import { ... }
    if matches!(tokens.get(i), Some(Token::LBrace)) {
        i += 1;

        while let Some(tok) = tokens.get(i) {
            match tok {
                Token::Ident(name) => {
                    named.push(name.clone());
                    i += 1;
                }
                Token::Comma => i += 1,
                Token::RBrace => {
                    i += 1;
                    break;
                }
                _ => return None,
            }
        }
    }

    // expect `from`
    if !matches!(tokens.get(i), Some(Token::From)) {
        return None;
    }
    i += 1;

    // expect source string
    if let Some(Token::String(src)) = tokens.get(i) {
        Some((
            Import {
                source: src.clone(),
                default,
                named,
                is_dynamic: false,
            },
            i + 1,
        ))
    } else {
        None
    }
}


fn parse_dynamic_import(tokens: &[Token], start: usize) -> Option<(Import, usize)> {
    let mut i = start + 1;

    if !matches!(tokens.get(i), Some(Token::LParen)) {
        return None;
    }
    i += 1;

    let src = match tokens.get(i) {
        Some(Token::String(s)) => s.clone(),
        _ => return None,
    };
    i += 1;

    if !matches!(tokens.get(i), Some(Token::RParen)) {
        return None;
    }

    Some((
        Import {
            source: src,
            default: None,
            named: Vec::new(),
            is_dynamic: true,
        },
        i + 1,
    ))
}


