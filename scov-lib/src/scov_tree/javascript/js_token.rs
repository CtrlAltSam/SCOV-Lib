#[derive(Debug, Clone, PartialEq)]
pub enum JsToken {
    Import,
    From,
    As,
    Star,
    Ident(String),
    String(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Semi,
}