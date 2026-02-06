use super::js_token::JsToken as Token;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // ---------- whitespace ----------
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // ---------- line comment ----------
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
            i += 2;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // ---------- block comment ----------
        if c == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < chars.len() {
                if chars[i] == '*' && chars[i + 1] == '/' {
                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // ---------- string literal ----------
        if c == '"' || c == '\'' {
            let quote = c;
            i += 1;
            let start = i;

            while i < chars.len() {
                if chars[i] == '\\' {
                    i += 2;
                    continue;
                }
                if chars[i] == quote {
                    break;
                }
                i += 1;
            }

            let value: String = chars[start..i].iter().collect();
            tokens.push(Token::String(value));
            i += 1;
            continue;
        }

        // ---------- punctuation ----------
        let token = match c {
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),
            ';' => Some(Token::Semi),
            _ => None,
        };

        if let Some(tok) = token {
            tokens.push(tok);
            i += 1;
            continue;
        }

        // ---------- identifier / keyword ----------
        if c.is_ascii_alphabetic() || c == '_' || c == '$' {
            let start = i;
            i += 1;

            while i < chars.len()
                && (chars[i].is_ascii_alphanumeric()
                    || chars[i] == '_'
                    || chars[i] == '$')
            {
                i += 1;
            }

            let ident: String = chars[start..i].iter().collect();
            match ident.as_str() {
                "import" => tokens.push(Token::Import),
                "from" => tokens.push(Token::From),
                _ => tokens.push(Token::Ident(ident)),
            }
            continue;
        }

        // ---------- everything else ----------
        i += 1;
    }

    tokens
}
