#[derive(Debug, Clone, PartialEq)]
pub enum JsToken {
    Import,
    From,
    Ident(String),
    String(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    Semi,
}