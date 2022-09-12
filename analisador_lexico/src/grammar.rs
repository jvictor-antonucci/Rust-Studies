#[derive(Clone)]
pub struct AlphabetItem {
    pub text: String,
    pub terminal: bool,
}

#[derive(Clone)]
pub struct GrammarRule {
    pub left: AlphabetItem,
    pub right: Vec<AlphabetItem>,
}

impl GrammarRule {
    pub fn show(&self) {
        println!(
            "{} -> {}",
            self.left.text,
            self.right
                .iter()
                .map(|item| String::from(&item.text))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

pub struct Grammar {
    rules: Vec<GrammarRule>,
}

impl Grammar {
    pub fn new() -> Grammar {
        let mut grammar = Grammar { rules: vec![] };
        grammar.init_rules();

        grammar
    }

    pub fn get_rule(&self, index: usize) -> GrammarRule {
        self.rules[index].clone()
    }

    pub fn show(&self) {
        for n in 0..self.rules.len() {
            print!("{}.", n);
            self.rules[n].show();
        }
    }

    fn init_rules(&mut self) {
        self.add_rule("P'", "P");
        self.add_rule("P", "inicio V A");
        self.add_rule("V", "varinicio LV");
        self.add_rule("LV", "D LV");
        self.add_rule("LV", "varfim pt_v");
        self.add_rule("D", "TIPO L pt_v");
        self.add_rule("L", "id vir L");
        self.add_rule("L", "id");
        self.add_rule("TIPO", "inteiro");
        self.add_rule("TIPO", "real");
        self.add_rule("TIPO", "literal");
        self.add_rule("A", "ES A");
        self.add_rule("ES", "leia id pt_v");
        self.add_rule("ES", "escreva ARG pt_v");
        self.add_rule("ARG", "lit");
        self.add_rule("ARG", "num");
        self.add_rule("ARG", "id");
        self.add_rule("A", "CMD A");
        self.add_rule("CMD", "id rcb LD pt_v");
        self.add_rule("LD", "OPRD opm OPRD");
        self.add_rule("LD", "OPRD");
        self.add_rule("OPRD", "id");
        self.add_rule("OPRD", "num");
        self.add_rule("A", "COND A");
        self.add_rule("COND", "CAB CP");
        self.add_rule("CAB", "se ab_p EXP_R fc_p então");
        self.add_rule("EXP_R", "OPRD opr OPRD");
        self.add_rule("CP", "ES CP");
        self.add_rule("CP", "CMD CP");
        self.add_rule("CP", "COND CP");
        self.add_rule("CP", "fimse");
        self.add_rule("A", "R A");
        self.add_rule("R", "CABR CPR");
        self.add_rule("CABR", "repita ab_p EXP_R fc_p");
        self.add_rule("CPR", "ES CPR");
        self.add_rule("CPR", "CMD CPR");
        self.add_rule("CPR", "COND CPR");
        self.add_rule("CPR", "fimrepita");
        self.add_rule("A", "fim");
    }

    fn add_rule(&mut self, left_str: &str, right_str: &str) {
        let left = AlphabetItem {
            text: String::from(left_str),
            terminal: false,
        };

        let mut right: Vec<AlphabetItem> = Vec::new();
        for item in right_str.split_whitespace() {
            right.push(AlphabetItem {
                text: String::from(item),
                terminal: item.to_lowercase().eq(item),
            });
        }

        self.rules.push(GrammarRule { left, right });
    }
}
