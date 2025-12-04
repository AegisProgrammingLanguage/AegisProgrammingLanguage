use serde_json::{json, Value};
use std::iter::Peekable;
use std::str::Chars;

// --- 1. LEXER (TOKENIZER) ---

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Mots-clés
    Var, If, Else, While, For, Func, Return, Print, Input, Class, New, Extends, Import, Break, Switch, Case, Default,
    // Identifiants et Littéraux
    Identifier(String),
    StringLiteral(String),
    Number(i64),
    // Symboles
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, Neq, Lt, Gt, LtEq, GtEq,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Dot, Colon,
    EOF,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { chars: input.chars().peekable() }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\n' | '\r' => { self.chars.next(); } // Skip whitespace
                '/' => {
                    self.chars.next();
                    if let Some(&'/') = self.chars.peek() {
                        // Commentaire: skip jusqu'à la fin de la ligne
                        while let Some(&c) = self.chars.peek() {
                            if c == '\n' { break; }
                            self.chars.next();
                        }
                    } else {
                        tokens.push(Token::Slash);
                    }
                }
                '{' => { tokens.push(Token::LBrace); self.chars.next(); }
                '}' => { tokens.push(Token::RBrace); self.chars.next(); }
                '(' => { tokens.push(Token::LParen); self.chars.next(); }
                ')' => { tokens.push(Token::RParen); self.chars.next(); }
                '[' => { tokens.push(Token::LBracket); self.chars.next(); }
                ']' => { tokens.push(Token::RBracket); self.chars.next(); }
                ',' => { tokens.push(Token::Comma); self.chars.next(); }
                '.' => { tokens.push(Token::Dot); self.chars.next(); }
                ':' => { tokens.push(Token::Colon); self.chars.next(); }
                '+' => { tokens.push(Token::Plus); self.chars.next(); }
                '-' => { tokens.push(Token::Minus); self.chars.next(); }
                '*' => { tokens.push(Token::Star); self.chars.next(); }
                '%' => { tokens.push(Token::Percent); self.chars.next(); }
                '=' => {
                    self.chars.next();
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::EqEq);
                    } else {
                        tokens.push(Token::Eq);
                    }
                }
                '!' => {
                    self.chars.next();
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::Neq);
                    } else {
                        panic!("Unexpected char '!'");
                    }
                }
                '<' => {
                    self.chars.next();
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::LtEq);
                    } else {
                        tokens.push(Token::Lt);
                    }
                }
                '>' => {
                    self.chars.next();
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next();
                        tokens.push(Token::GtEq);
                    } else {
                        tokens.push(Token::Gt);
                    }
                }
                '"' => tokens.push(self.read_string()),
                c if c.is_digit(10) => tokens.push(self.read_number()),
                c if c.is_alphabetic() || c == '_' => tokens.push(self.read_identifier()),
                _ => panic!("Unexpected char '{}'", c),
            }
        }
        tokens.push(Token::EOF);
        tokens
    }

    fn read_string(&mut self) -> Token {
        self.chars.next(); // Consume opening quote
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                self.chars.next(); // Consume closing quote
                return Token::StringLiteral(s);
            }
            s.push(self.chars.next().unwrap());
        }
        panic!("Unterminated string");
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_digit(10) {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        Token::Number(s.parse().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        match s.as_str() {
            "var" => Token::Var,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "func" => Token::Func,
            "return" => Token::Return,
            "print" => Token::Print,
            "input" => Token::Input,
            "class" => Token::Class,
            "new" => Token::New,
            "extends" => Token::Extends,
            "import" => Token::Import,
            "break" => Token::Break,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "default" => Token::Default,
            _ => Token::Identifier(s),
        }
    }
}

// --- 2. PARSER ---

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Value, String> {
        let mut instructions = Vec::new();
        while !self.is_at_end() {
            instructions.push(self.parse_statement()?);
        }
        Ok(json!(instructions))
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }
        &self.tokens[self.pos - 1]
    }

    fn match_token(&mut self, token: Token) -> bool {
        if self.check(&token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() { return false; }
        // Comparaison approximative car Token::Identifier porte une donnée
        std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
    }

    fn is_at_end(&self) -> bool {
        self.peek() == &Token::EOF
    }

    fn consume(&mut self, expected: Token, msg: &str) -> Result<&Token, String> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            Err(format!("{} (Got {:?})", msg, self.peek()))
        }
    }

    // --- Statements ---

    fn parse_statement(&mut self) -> Result<Value, String> {
        match self.peek() {
            Token::Var => self.parse_var(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::While => self.parse_while(),
            Token::Func => self.parse_func(),
            Token::Return => self.parse_return(),
            Token::Input => self.parse_input(),
            Token::Break => { self.advance(); Ok(json!(["break"])) },
            Token::For => self.parse_for(),
            
            // Gestion des assignations ou appels isolés
            Token::Identifier(_) => {
                // Lookahead pour voir si c'est une assignation
                if self.pos + 1 < self.tokens.len() && self.tokens[self.pos + 1] == Token::Eq {
                    let name = if let Token::Identifier(n) = self.advance() { n.clone() } else { unreachable!() };
                    self.advance(); // Eat '='
                    let expr = self.parse_expression()?;
                    Ok(json!(["set", name, expr]))
                } else {
                    self.parse_expression()
                }
            },
            _ => Err(format!("Unexpected token at start of statement: {:?}", self.peek())),
        }
    }

    fn parse_block(&mut self) -> Result<Value, String> {
        self.consume(Token::LBrace, "Expect '{' before block")?;
        let mut block = Vec::new();
        while !self.check(&Token::RBrace) && !self.is_at_end() {
            block.push(self.parse_statement()?);
        }
        self.consume(Token::RBrace, "Expect '}' after block")?;
        Ok(json!(block))
    }

    fn parse_var(&mut self) -> Result<Value, String> {
        self.advance(); // Eat 'var'
        let name = if let Token::Identifier(n) = self.advance() { n.clone() } else { return Err("Expect var name".into()); };
        
        let expr = if self.match_token(Token::Eq) {
            self.parse_expression()?
        } else {
            json!(null)
        };
        Ok(json!(["set", name, expr]))
    }

    fn parse_print(&mut self) -> Result<Value, String> {
        self.advance();
        let expr = self.parse_expression()?;
        Ok(json!(["print", expr]))
    }

    fn parse_input(&mut self) -> Result<Value, String> {
        self.advance();
        let name = if let Token::Identifier(n) = self.advance() { n.clone() } else { return Err("Expect var name".into()); };
        let prompt = self.parse_expression()?;
        Ok(json!(["input", name, prompt]))
    }

    fn parse_if(&mut self) -> Result<Value, String> {
        self.advance();
        self.consume(Token::LParen, "Expect '('")?;
        let condition = self.parse_expression()?;
        self.consume(Token::RParen, "Expect ')'")?;
        
        let true_block = self.parse_block()?;
        let mut false_block = json!([]);

        if self.match_token(Token::Else) {
            if self.check(&Token::If) {
                false_block = json!([self.parse_if()?]);
            } else {
                false_block = self.parse_block()?;
            }
        }
        
        // Si false_block est vide, on renvoie une liste à 3 éléments, sinon 4
        if false_block.as_array().unwrap().is_empty() {
             Ok(json!(["if", condition, true_block]))
        } else {
             Ok(json!(["if", condition, true_block, false_block]))
        }
    }

    fn parse_while(&mut self) -> Result<Value, String> {
        self.advance();
        self.consume(Token::LParen, "Expect '('")?;
        let cond = self.parse_expression()?;
        self.consume(Token::RParen, "Expect ')'")?;
        let body = self.parse_block()?;
        Ok(json!(["while", cond, body]))
    }

    fn parse_for(&mut self) -> Result<Value, String> {
        self.advance(); // Mange le mot-clé 'for'
        self.consume(Token::LParen, "Expect '(' after for")?;
        
        // 1. Variable d'itération
        let var_name = if let Token::Identifier(n) = self.advance() { 
            n.clone() 
        } else { 
            return Err("Expect variable name in for loop".into()); 
        };
        
        self.consume(Token::Comma, "Expect ',' after variable")?;
        
        // 2. Start
        let start = self.parse_expression()?;
        self.consume(Token::Comma, "Expect ',' after start")?;
        
        // 3. End
        let end = self.parse_expression()?;
        self.consume(Token::Comma, "Expect ',' after end")?;
        
        // 4. Step
        let step = self.parse_expression()?;
        
        self.consume(Token::RParen, "Expect ')' after for arguments")?;
        
        // 5. Body
        let body = self.parse_block()?;
        
        // Génère l'instruction JSON que le runtime Rust attend :
        // ["for_range", "var_name", start, end, step, [body]]
        Ok(json!(["for_range", var_name, start, end, step, body]))
    }

    fn parse_return(&mut self) -> Result<Value, String> {
        self.advance();
        let expr = self.parse_expression()?;
        Ok(json!(["return", expr]))
    }

    fn parse_func(&mut self) -> Result<Value, String> {
        self.advance();
        let name = if let Token::Identifier(n) = self.advance() { n.clone() } else { return Err("Expect func name".into()); };
        
        self.consume(Token::LParen, "Expect '('")?;
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                if let Token::Identifier(p) = self.advance() { params.push(p.clone()); }
                if !self.match_token(Token::Comma) { break; }
            }
        }
        self.consume(Token::RParen, "Expect ')'")?;
        let body = self.parse_block()?;
        
        Ok(json!(["function", name, params, body]))
    }

    // --- Expressions (Pratt Parsing simplifié ou Recursive Descent) ---
    // Pour simplifier, on fait: Logic > Additive > Multiplicative > Primary

    fn parse_expression(&mut self) -> Result<Value, String> {
        self.parse_logic()
    }

    fn parse_logic(&mut self) -> Result<Value, String> {
        let mut left = self.parse_additive()?;
        
        while let Token::EqEq | Token::Lt | Token::Gt = self.peek() {
            let op = match self.advance() {
                Token::EqEq => "==",
                Token::Lt => "<",
                Token::Gt => ">",
                _ => unreachable!()
            };
            let right = self.parse_additive()?;
            left = json!([op, left, right]);
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Value, String> {
        let mut left = self.parse_multiplicative()?;
        while let Token::Plus | Token::Minus = self.peek() {
            let op = match self.advance() {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => unreachable!()
            };
            let right = self.parse_multiplicative()?;
            left = json!([op, left, right]);
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Value, String> {
        let mut left = self.parse_primary()?;
        while let Token::Star | Token::Slash = self.peek() {
            let op = match self.advance() {
                Token::Star => "*",
                Token::Slash => "/",
                _ => unreachable!()
            };
            let right = self.parse_primary()?;
            left = json!([op, left, right]);
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Value, String> {
        match self.peek() {
            Token::Number(n) => { let v = *n; self.advance(); Ok(json!(v)) },
            Token::StringLiteral(s) => { let v = s.clone(); self.advance(); Ok(json!(v)) },
            Token::LBracket => {
                self.advance(); // Mange le '['
                let mut elements = Vec::new();
                
                // Si la liste n'est pas vide (pas ']')
                if !self.check(&Token::RBracket) {
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(Token::Comma) { break; }
                    }
                }
                
                self.consume(Token::RBracket, "Expect ']' after list")?;
                
                // Retourne un tableau JSON valide
                Ok(json!(elements))
            },
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                
                // Appel de fonction ?
                if self.match_token(Token::LParen) {
                    let mut args = Vec::new();
                    if !self.check(&Token::RParen) {
                        loop {
                            args.push(self.parse_expression()?);
                            if !self.match_token(Token::Comma) { break; }
                        }
                    }
                    self.consume(Token::RParen, "Expect ')'")?;
                    
                    // Liste Blanche Native (très important !)
                    let native_commands = vec!["to_int", "len", "str"];
                    if native_commands.contains(&name.as_str()) {
                        let mut call = vec![json!(name)];
                        call.extend(args);
                        Ok(json!(call))
                    } else {
                        let mut call = vec![json!("call"), json!(name)];
                        call.extend(args);
                        Ok(json!(call))
                    }
                } else {
                    Ok(json!(["get", name]))
                }
            },
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(Token::RParen, "Expect ')'")?;
                Ok(expr)
            },
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek())),
        }
    }
}

pub fn compile(source: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse()
}