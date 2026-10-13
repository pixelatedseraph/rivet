use std::{fs::{self, File}, iter::Map};

use crate::lexer::Token::GTEQ;

/*
    Those nested while let Some(&ch) = revo_as_chars.next() cases have a nasty bug,fix it asap    
*/

#[derive(Debug)]
pub enum Token{
    //basic keywords
    Let,
    Fn,
    Const,
    If,
    Else,
    For,
    In,
    Match,
    Type,
    Do,
    End,

    //operators
    Plus,
    Minus,
    Star,
    Slash,
    Lparent,
    RParent,
    LSqBrkt,
    RSqBrkt,
    LBrace,
    RBrace,
    Colon,
    Dot,
    QuestionMark,
    Eq,DoubleEq,
    GT,LT,
    GTEQ,LTEQ,
    Pipe,
    Guard,
    Arrow,
    Tilde,
    Quote,
    Comma,

    Ident(String,bool),
} 
pub struct Lexer{
    pub tokens: Vec<Token>,
    pub revosource: String,
    pub index: usize,
}

impl Lexer{
    pub fn new(path: &str) -> Self{
        let sourcecode = fs::read_to_string(path).unwrap();
        Lexer { tokens: (Vec::new()), revosource: (sourcecode), index: (1) }
    }

    pub fn tokenize(&mut self){
        let mut revo_as_chars = self.revosource.chars().peekable();
        while let Some(&ch) = revo_as_chars.peek(){
            match ch {
                ' ' | '\n' | '\t' | '\r' =>{
                    revo_as_chars.next();
                    self.index+=1;
                }
                '0'..='9' =>{
                    let mut number = String::new();
                    number.push(ch);
                    while let Some(&ch) = revo_as_chars.peek(){
                        if ch.is_ascii_digit(){
                            number.push(ch);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                        else{
                            break;
                        }
                    }
                    self.tokens.push(Token::Ident(number,true));
                    revo_as_chars.next();
                    self.index+=1;
                } 
                'a'..='z' | 'A'..='Z' | '_'  =>{
                    let mut identifier = String::new();
                    while let Some(&ch) = revo_as_chars.peek() {
                        if ch.is_ascii_alphanumeric() || ch == '_'{
                            identifier.push(ch);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                        else{
                            break;
                        }
                    }
                    if identifier == "let"{
                        self.tokens.push(Token::Let);
                    }
                    else if identifier == "fn"{
                        self.tokens.push(Token::Fn);
                    }
                    else if identifier == "do"{
                        self.tokens.push(Token::Do);
                    }
                    else if identifier == "in"{
                        self.tokens.push(Token::In);
                    }
                    else if identifier == "end"{
                        self.tokens.push(Token::End);
                    }
                    else if identifier == "if"{
                        self.tokens.push(Token::If);
                    }
                    else if identifier == "else"{
                        self.tokens.push(Token::Else);
                    }
                    else if identifier == "const"{
                        self.tokens.push(Token::Const);
                    }
                    else if identifier == "for"{
                        self.tokens.push(Token::For);
                    }
                    else if identifier == "match"{
                        self.tokens.push(Token::Match);
                    }
                    else if identifier == "type"{
                        self.tokens.push(Token::Type);
                    }
                    else{
                        self.tokens.push(Token::Ident(identifier,false));
                    }
                    revo_as_chars.next();
                    self.index+=1;
                }
                '=' => {
                    revo_as_chars.next();
                    if let Some(&ch) = revo_as_chars.peek(){
                        match ch {
                            '=' => {
                                self.tokens.push(Token::DoubleEq);
                                revo_as_chars.next();
                                self.index +=1;
                            }
                            '>' =>{
                                self.tokens.push(Token::Arrow);
                                revo_as_chars.next();
                                self.index +=1;
                            }
                            _ => {
                                self.tokens.push(Token::Eq);
                                self.index+=1;
                            }
                        }
                    }
                }
                '+' =>{
                    self.tokens.push(Token::Plus);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '-' => {
                    self.tokens.push(Token::Minus);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '*' => {
                    self.tokens.push(Token::Star);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '/' => {
                    self.tokens.push(Token::Slash);
                    revo_as_chars.next();
                    self.index+=1;
                }
                ':' => {
                    self.tokens.push(Token::Colon);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '?' => {
                    self.tokens.push(Token::QuestionMark);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '~' => {
                    self.tokens.push(Token::Tilde);
                    revo_as_chars.next();
                    self.index+=1;
                }
                ',' =>{
                    self.tokens.push(Token::Comma);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '"' =>{
                    let mut identifier = String::new();
                    while let Some(&ch) = revo_as_chars.peek(){
                        if ch == '"'{
                            break;
                        }
                        else{
                            identifier.push(ch);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                    }
                    self.tokens.push(Token::Ident(identifier,false));
                    revo_as_chars.next();
                    self.index+=1;
                }
                '.' =>{
                    self.tokens.push(Token::Dot);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '|' => {
                    if let Some(&ch) = revo_as_chars.peek(){
                        if ch == '>'{
                            self.tokens.push(Token::Pipe);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                        else{
                            self.tokens.push(Token::Guard);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                    }
                }
                '>' => {
                    if let Some(&ch) = revo_as_chars.peek(){
                        if ch == '='{
                            self.tokens.push(Token::GTEQ);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                        else{
                            self.tokens.push(Token::GT);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                    }
                }
                '<' => {
                    if let Some(&ch) = revo_as_chars.peek(){
                        if ch == '='{
                            self.tokens.push(Token::LTEQ);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                        else{
                            self.tokens.push(Token::LT);
                            revo_as_chars.next();
                            self.index+=1;
                        }
                    }
                }
                '(' => {
                    self.tokens.push(Token::Lparent);
                    revo_as_chars.next();
                    self.index+=1;
                }
                ')' =>{
                    self.tokens.push(Token::RParent);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '{' =>{
                    self.tokens.push(Token::LBrace);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '}' =>{
                    self.tokens.push(Token::RBrace);
                    revo_as_chars.next();
                    self.index+=1;
                }
                '[' =>{
                    self.tokens.push(Token::LSqBrkt);
                    revo_as_chars.next();
                    self.index+=1;
                }
                ']' =>{
                    self.tokens.push(Token::RSqBrkt);
                    revo_as_chars.next();
                    self.index+=1;
                }
            
                _ => {panic!("Unexpected character: '{ch}' at {}",self.index);}
            }
        }
    }
}



