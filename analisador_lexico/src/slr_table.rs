use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
pub enum SLRAction {
    S(u8),
    R(u8),
    Acc,
    E(u8),
}

impl SLRAction {
    fn from_str(s: &str) -> SLRAction {
        let mut chars = s.chars();
        let kind = chars.next();
        let n = chars.as_str().parse::<u8>().unwrap_or(0);

        match kind {
            Some('S') | Some('s') => SLRAction::S(n),
            Some('R') | Some('r') => SLRAction::R(n),
            Some('A') | Some('a') => SLRAction::Acc,
            Some('E') | Some('e') => SLRAction::E(n),
            _ => SLRAction::E(0),
        }
    }
}

pub struct ActionTable {
    table: HashMap<(u8, String), SLRAction>,
}

impl ActionTable {
    pub fn new() -> ActionTable {
        let terminals = [
            "inicio",
            "varinicio",
            "varfim",
            "pt_v",
            "id",
            "vir",
            "inteiro",
            "real",
            "literal",
            "leia",
            "escreva",
            "lit",
            "num",
            "rcb",
            "opm",
            "se",
            "ab_p",
            "fc_p",
            "entao",
            "opr",
            "fimse",
            "repita",
            "fimrepita",
            "fim",
            "eof",
        ];

        let path = "./src/action_table.csv";
        let actions_file = match File::open(path) {
            Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
            Ok(file) => file,
        };

        let mut lines = io::BufReader::new(actions_file).lines();

        lines.next();

        let mut table = HashMap::new();

        for line in lines {
            if let Ok(okline) = line {
                let actions = okline.split(",").collect::<Vec<&str>>();
                let state = actions[0].parse::<u8>().unwrap();

                for i in 0..terminals.len() {
                    let action = actions[i + 1];
                    let terminal = terminals[i];
                    table.insert((state, String::from(terminal)), SLRAction::from_str(action));
                }
            }
        }

        ActionTable { table }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn show(&self) {
        for key in self.table.keys() {
            println!("{:?} -> {:?}", key, self.table.get(key).unwrap());
        }
    }

    pub fn get(&self, key: &(u8, String)) -> SLRAction {
        self.table.get(key).unwrap().clone()   
    }
}

pub struct GotoTable {
    table: HashMap<(u8, String), u8>,
}

impl GotoTable {
    pub fn new() -> GotoTable {
        let non_terminals = [
            "P'",
            "P", 
            "V", 
            "LV", 
            "D", 
            "L", 
            "TIPO", 
            "A", 
            "ES", 
            "ARG", 
            "CMD", 
            "LD", 
            "OPRD", 
            "COND", 
            "CAB",
            "EXP_R", 
            "CP", 
            "R", 
            "CABR", 
            "CPR",
        ];

        let path = "./src/goto_table.csv";
        let goto_file = match File::open(path) {
            Err(_) => panic!("Não foi possível abrir o arquivo {}", path),
            Ok(file) => file,
        };

        let mut lines = io::BufReader::new(goto_file).lines();

        lines.next();

        let mut table = HashMap::new();

        for line in lines {
            if let Ok(okline) = line {
                let gotos = okline.split(",").collect::<Vec<&str>>();
                let state = gotos[0].parse::<u8>().unwrap();

                for i in 0..non_terminals.len() {
                    let goto = gotos[i + 1].parse::<u8>().unwrap();
                    if goto == 0 {
                        continue;
                    }
                    let non_terminal = non_terminals[i];
                    table.insert((state, String::from(non_terminal)), goto);
                }
            }
        }

        GotoTable { table }
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn show(&self) {
        for key in self.table.keys() {
            println!("{:?} -> {}", key, self.table.get(key).unwrap());
        }
    }

    pub fn get(&self, key: &(u8, String)) -> u8 {
        self.table.get(key).unwrap().clone()
    }
}
