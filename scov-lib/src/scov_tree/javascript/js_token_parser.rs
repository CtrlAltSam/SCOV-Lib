use super::js_token::JsToken as Token;

#[derive(Debug, Clone)]
pub struct Import {
    pub source: String,
    pub default: Option<String>,
    pub named: Vec<NamedImport>,
    pub namespace: Option<String>,
    pub is_dynamic: bool,
}

#[derive(Debug, Clone)]
pub struct NamedImport {
    pub imported: String,
    pub local: String,
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
    let mut namespace = None;

    // import "mod";
    if let Some(Token::String(src)) = tokens.get(i) {
        return Some((
            Import {
                source: src.clone(),
                default: None,
                named,
                namespace,
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

    // optional comma
    if matches!(tokens.get(i), Some(Token::Comma)) {
        i += 1;
    }

    // import * as ns
    if matches!(tokens.get(i), Some(Token::Star)) {
        i += 1;

        if !matches!(tokens.get(i), Some(Token::As)) {
            return None;
        }
        i += 1;

        if let Some(Token::Ident(name)) = tokens.get(i) {
            namespace = Some(name.clone());
            i += 1;
        } else {
            return None;
        }
    }

    // import { ... }
    if matches!(tokens.get(i), Some(Token::LBrace)) {
        i += 1;

        while let Some(tok) = tokens.get(i) {
            match tok {
                Token::Ident(imported) => {
                    let mut local = imported.clone();
                    i += 1;

                    // foo as bar
                    if matches!(tokens.get(i), Some(Token::As)) {
                        i += 1;
                        if let Some(Token::Ident(alias)) = tokens.get(i) {
                            local = alias.clone();
                            i += 1;
                        } else {
                            return None;
                        }
                    }

                    named.push(NamedImport {
                        imported: imported.clone(),
                        local,
                    });
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

    // expect source
    if let Some(Token::String(src)) = tokens.get(i) {
        Some((
            Import {
                source: src.clone(),
                default,
                named,
                namespace,
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
            namespace: None,
        },
        i + 1,
    ))
}


