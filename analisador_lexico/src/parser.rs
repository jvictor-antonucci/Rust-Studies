use crate::{
    class::Class,
    grammar::Grammar,
    scanner::Scanner,
    slr_table::{ActionTable, GotoTable, SLRAction},
    token::Token,
};

struct SyntaticStack {
    stack: Vec<u8>,
}

impl SyntaticStack {
    fn new() -> SyntaticStack {
        SyntaticStack { stack: vec![0] }
    }

    fn top(&self) -> u8 {
        self.stack[self.stack.len() - 1]
    }

    fn push(&mut self, n: u8) {
        self.stack.push(n);
    }

    fn pop(&mut self, count: u8) {
        for _ in 0..count {
            self.stack.pop();
        }
    }
}

pub struct Parser {
    syntatic_stack: SyntaticStack,
    grammar: Grammar,
    action_table: ActionTable,
    goto_table: GotoTable,
    token_buffer: Vec<Token>,
    error_messages: Vec<String>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            syntatic_stack: SyntaticStack::new(),
            grammar: Grammar::new(),
            action_table: ActionTable::new(),
            goto_table: GotoTable::new(),
            token_buffer: Vec::new(),
            error_messages: Vec::new(),
        }
    }

    pub fn parse(&mut self, scanner: &mut Scanner) {
        let mut token = self.next_token(scanner);
        let mut a = token.class.clone();

        loop {
            let s = self.syntatic_stack.top();
            let action = self.action_table.get(&(s, Class::to_str(a.clone())));

            match action {
                SLRAction::S(t) => {
                    self.syntatic_stack.push(t);
                    token = self.next_token(scanner);
                    a = token.class.clone();
                }
                SLRAction::R(r) => {
                    let rule = self.grammar.get_rule(r as usize);
                    rule.show();
                    #[allow(non_snake_case)]
                    let A = rule.left;
                    let beta = rule.right;
                    self.syntatic_stack.pop(beta.len() as u8);
                    let t = self.syntatic_stack.top();
                    self.syntatic_stack
                        .push(self.goto_table.get(&(t, A.text.clone())));
                }
                SLRAction::Acc => break,
                SLRAction::E(e) => {
                    self.token_buffer.push(token);

                    if !self.error_recovery(e, scanner) {
                        break;
                    }

                    token = self.next_token(scanner);
                    a = token.class.clone();
                }
            }
        }

        scanner.show_error_messages();
        self.show_error_messages();
    }

    fn next_token(&mut self, scanner: &mut Scanner) -> Token {
        if self.token_buffer.is_empty() {
            return scanner.safe_scan();
        } else {
            return self.token_buffer.pop().unwrap();
        }
    }

    fn error_recovery(&mut self, error_code: u8, scanner: &mut Scanner) -> bool {
        const MAX_SYNTATIC_ERROR_COUNTER: u8 = 100;
        static mut SYNTATIC_ERRORS: u8 = 0;

        unsafe {
            SYNTATIC_ERRORS += 1;
            if SYNTATIC_ERRORS > MAX_SYNTATIC_ERROR_COUNTER {
                return false;
            }
        }

        match error_code {
            1 => {
                self.token_buffer.clear();
                self.token_buffer.push(Token::new_from_lexeme("eof"));

                self.error_messages.push(format!("[ES1] Erro na linha {}, coluna {}: nenhum código deve vir após a palavra reservada 'fim'", scanner.get_row(), scanner.get_col()));

                return true;
            }
            2 => {
                self.token_buffer.push(Token::new(
                    Some(Class::Ptv),
                    Some(String::from(";")),
                    None,
                ));

                self.error_messages.push(format!(
                    "[ES2] Erro sintático na linha {}, coluna {}: ausência de ';'",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            3 => {
                self.token_buffer.pop();

                self.error_messages.push(format!(
                    "[ES3] Erro sintático na linha {}, coluna {}: múltiplos ';' na sequência",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            4 => {
                let token = self.token_buffer.pop();

                self.error_messages.push(format!(
                    "[ES4] Erro sintático na linha {}, coluna {}: token inválido após um ';'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            5 => {
                let token = self.token_buffer.pop();

                self.token_buffer.push(Token::new(
                    Some(Class::Abp),
                    Some(String::from("(")),
                    None,
                ));

                self.error_messages.push(format!(
                    "[ES5] Erro sintático na linha {}, coluna {}: esperado um '(' após a palavra reservada 'se'\n    NOTA: o token '{}' foi removido",
                    scanner.get_row(),
                    scanner.get_col(),
                    token.unwrap().lexeme.unwrap()
                ));
                return true;
            }
            6 => {
                self.token_buffer.push(Token::new(
                    Some(Class::Abp),
                    Some(String::from("(")),
                    None,
                ));

                self.error_messages.push(format!(
                    "[ES6] Erro sintático na linha {}, coluna {}: esperado um '(' após a palavra reservada 'se'",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return true;
            }
            7 => {
                self.error_messages.push(format!(
                    "[ES7] Erro sintático na linha {}, coluna {}: após um identificador deve vir um operador relacional, um operador aritimético, um ')' ou um ';'\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
            8 => {
                self.error_messages.push(format!(
                    "[ES8] Erro sintático na linha {}, coluna {}: após um número deve vir um operador relacional, um operador aritimético, um ')' ou um ';'\n    NOTA: não é possível recuperar deste erro e portanto a análise foi interrompida",
                    scanner.get_row(),
                    scanner.get_col()
                ));
                return false;
            }
            _ => {
                self.error_messages.push(format!("[ES0] Erro na linha {}, coluna {}\n [ALERTA] Não é possível recuperar deste erro, portanto a análise foi interrompida", scanner.get_row(), scanner.get_col()));

                return false;
            }
        }
    }

    fn show_error_messages(&self) -> u8 {
        let n = self.error_messages.len();
        match n {
            0 => (),
            1 => println!("Foi encontrado 1 erro sintático"),
            _ => println!("Foi encontrado {} erros sintáticos", n),
        }

        for i in 0..n {
            let msg = &self.error_messages[i];
            println!("# ERRO {}", i + 1);
            println!("    {}", msg);
        }

        n as u8
    }
}
