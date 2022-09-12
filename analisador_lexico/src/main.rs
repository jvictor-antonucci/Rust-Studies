use std::fs::File;

use analisador_lexico::{scanner::Scanner, parser::Parser};

fn main() {
    let path: &str =
        "./src/source_code.txt";
    let file: File = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Não é possível abrir o arquivo: {}", path),
    };

    let mut scanner: Scanner = Scanner::new(file);

    let mut parser = Parser::new();
    parser.parse(&mut scanner);
    
    // loop {
    //     let token = scanner.scan();

    //     let class = Class::to_str(token.class.clone());
    //     let lexeme = String::from(token.lexeme.unwrap_or_else(|| String::from("Null")));
    //     let token_type = TokenType::to_str(token.token_type.clone());
        
    //     println!("Classe: {}, Lexema: {}, Tipo: {}", class, lexeme, token_type);

    //     if token.class.eq(&Some(Class::Eof)) {
    //         break;
    //     }
    // }

    // println!("\n\n<<--------- TABELA DE SIMBOLOS --------->>\n\n");
    // scanner.show_symbol_table();
}
