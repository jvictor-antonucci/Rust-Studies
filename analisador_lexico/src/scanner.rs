use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    class::Class,
    lexical_afd::{AFDState, Action, AFD},
    symbol_table::SymbolTable,
    token::Token,
    token_type::TokenType,
};

pub struct Scanner {
    file: BufReader<File>,
    line: Vec<char>,
    cursor: (usize, usize),
    pub symbol_table: SymbolTable,
    error_messages: Vec<String>,
}

impl Scanner {
    pub fn new(file: File) -> Scanner {
        let file: BufReader<File> = BufReader::new(file);
        let line: Vec<char> = Vec::new();
        let cursor: (usize, usize) = (0, 0);
        let symbol_table: SymbolTable = SymbolTable::new();
        let error_messages: Vec<String> = Vec::new();

        Scanner {
            file,
            line,
            cursor,
            symbol_table,
            error_messages,
        }
    }

    pub fn show_symbol_table(&self) {
        for (lexeme, token) in self.symbol_table.iter() {
            let class = Class::to_str(token.class.clone());
            let token_type = TokenType::to_str(token.token_type.clone());

            println!("Classe: {}, Lexema: {}, Tipo: {}", class, lexeme, token_type);
        }
    }

    pub fn scan(&mut self) -> Token {
        let mut lexeme = String::new();
        let mut afd = AFD::new();

        while let Some(c) = self.read_char() {
            afd.advance(c);

            match afd.action {
                Action::GoBack => self.go_back(),
                Action::Standard => lexeme.push(c),
                Action::Clear => lexeme.clear(),
                Action::ShowErrMessage => {
                    self.insert_err_message(c, &afd.state);
                    println!("{:?}", self.error_messages.last().unwrap());
                }
                Action::Idle => (),
            }

            if afd.done {
                return self.assemble_token(lexeme, afd.state);
            }
        }

        if lexeme.len() > 0 {
            match afd.state {
                AFDState::Accept(_) => return self.assemble_token(lexeme, afd.state),
                AFDState::NonAccept(_) => {
                    afd.state = AFDState::Error(6);
                    self.insert_err_message(' ', &afd.state);
                    println!("{:?}", self.error_messages.last().unwrap());

                    return self.assemble_token(lexeme, afd.state);
                }
                _ => (),
            }
        }

        Token::new(Some(Class::Eof), Some(String::from("EOF")), None)
    }

    fn read_char(&mut self) -> Option<char> {
        static mut IS_EOF: bool = false;

        if self.cursor.1 == self.line.len() {
            self.cursor.0 += 1;
            self.cursor.1 = 0;
            let mut s = String::new();
            match self.file.read_line(&mut s) {
                Ok(0) => unsafe {
                    IS_EOF = true;
                },
                Ok(_) => self.line = s.chars().collect(),
                Err(_) => (),
            }
        }

        unsafe {
            if IS_EOF {
                return None;
            }
        }

        let c = self.line[self.cursor.1];
        self.cursor.1 += 1;

        Some(c)
    }

    fn go_back(&mut self) {
        self.cursor.1 -= 1;
    }

    fn insert_err_message(&mut self, c: char, afd_state: &AFDState) {
        let row = self.cursor.0;
        let col = self.cursor.1;

        match afd_state {
            AFDState::Error(0) => self.error_messages.push(format!(
                "Erro Léxico 0: {:?} não pertence ao alfabeto. Linha [{}] Coluna [{}]",
                c, row, col
            )),
            AFDState::Error(1) => self.error_messages.push(format!("Erro Léxico 1: {:?} não é início de nenhum token. Linha [{}] Coluna [{}]", c, row, col)),
            AFDState::Error(2) => self.error_messages.push(format!("Erro Léxico 2: após um '.' em um [num] deve-se conter um dígito - {:?} foi encontrado. Linha [{}] Coluna [{}]", c, row, col)),
            AFDState::Error(3) => self.error_messages.push(format!("Erro Léxico 3: após um 'e' ou 'E' em um [num] deve-se conter um dígito, um '+' ou um '-' - {:?} foi encontrado. Linha [{}] Coluna [{}]", c, row, col)),
            AFDState::Error(4) => self.error_messages.push(format!("Erro Léxico 4: após um 'e+' ou 'E+' em um [num] dev-se conter um dígito - {:?} foi encontrado. Linha [{}] Coluna [{}]", c, row, col)),
            AFDState::Error(5) => self.error_messages.push(format!("Erro Léxico 5: após um 'e-' ou 'E-' em um [num] dev-se conter um dígito - {:?} foi encontrado. Linha [{}] Coluna [{}]", c, row, col)),
            AFDState::Error(6) => self.error_messages.push(format!("Erro Léxico 6: não foi encontrado o fechamento do comentário ou literal. Linha [{}] Coluna [{}]", row, col)),
            _ => (),
        }
    }

    fn assemble_token(&mut self, lexeme: String, afd_state: AFDState) -> Token {
        let mut class: Option<Class> = None;
        let mut lexeme = Some(lexeme);
        let mut token_type: Option<TokenType> = None;

        match afd_state {
            AFDState::Accept(1) => {
                class = Some(Class::Num);
                token_type = Some(TokenType::Inteiro);
            }
            AFDState::Accept(3) => {
                class = Some(Class::Num);
                token_type = Some(TokenType::Real);
            }
            AFDState::Accept(6) => {
                class = Some(Class::Num);
                token_type = Some(TokenType::Real);
            }
            AFDState::Accept(8) => {
                class = Some(Class::Lit);
                token_type = Some(TokenType::Literal);
            }
            AFDState::Accept(9) => {
                class = Some(Class::Id);
                let lexeme_clone = lexeme.clone();
                if let Some(token) = self.symbol_table.get(lexeme_clone.unwrap()) {
                    return token;
                }

                let lexeme_clone = lexeme.clone();
                let token = Token::new(class.clone(), Some(lexeme_clone.clone().unwrap()), None);
                self.symbol_table.insert(lexeme_clone.unwrap(), token);
            }
            AFDState::Accept(13)
            | AFDState::Accept(15)
            | AFDState::Accept(16)
            | AFDState::Accept(17) => class = Some(Class::Opr),
            AFDState::Accept(14) | AFDState::Accept(18) => class = Some(Class::Rcb),
            AFDState::Accept(19) => class = Some(Class::Opm),
            AFDState::Accept(20) => class = Some(Class::Abp),
            AFDState::Accept(21) => class = Some(Class::Fcp),
            AFDState::Accept(22) => class = Some(Class::Ptv),
            AFDState::Accept(23) => class = Some(Class::Vir),
            AFDState::Accept(25) => {
                class = Some(Class::Num);
                token_type = Some(TokenType::Inteiro);
            }
            AFDState::Error(_) => {
                class = Some(Class::Erro);
                lexeme = None;
            }
            _ => (),
        };
        
        Token::new(class, lexeme, token_type)
    }
}
