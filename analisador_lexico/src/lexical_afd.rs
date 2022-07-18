pub enum AFDState {
    Initial,
    Accept(u8),
    NonAccept(u8),
    Error(u8),
}

pub enum Action {
    Idle,
    GoBack,
    Standard,
    Clear,
    ShowErrMessage,
}

pub struct AFD {
    pub state: AFDState,
    pub done: bool,
    pub action: Action,
}

impl AFD {
    pub fn new() -> AFD {
        AFD {
            state: AFDState::Initial,
            done: false,
            action: Action::Idle,
        }
    }

    pub fn advance(&mut self, c: char) {
        self.action = Action::Standard;

        match self.state {
            AFDState::Initial => match c {
                '0'..='9' => self.state = AFDState::Accept(1),
                '"' => self.state = AFDState::NonAccept(7),
                'a'..='z' | 'A'..='Z' => self.state = AFDState::Accept(9),
                '{' => self.state = AFDState::NonAccept(10),
                '<' => self.state = AFDState::Accept(13),
                '>' => self.state = AFDState::Accept(16),
                '=' => {
                    self.done = true;
                    self.state = AFDState::Accept(18);
                }
                '+' | '-' | '*' | '/' => {
                    self.done = true;
                    self.state = AFDState::Accept(19);
                }
                '(' => {
                    self.done = true;
                    self.state = AFDState::Accept(20);
                }
                ')' => {
                    self.done = true;
                    self.state = AFDState::Accept(21);
                }
                ';' => {
                    self.done = true;
                    self.state = AFDState::Accept(22);
                }
                ',' => {
                    self.done = true;
                    self.state = AFDState::Accept(23);
                }
                '\n' | '\r' | ' ' => {
                    self.state = AFDState::Initial;
                    self.action = Action::Idle;
                }
                c if is_valid(c) => self.err(1),
                _ => self.err(0),
            },
            AFDState::Accept(1) => match c {
                '0'..='9' => self.state = AFDState::Accept(1),
                '.' => self.state = AFDState::NonAccept(2),
                'e' | 'E' => self.state = AFDState::NonAccept(4),
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::Accept(3) => match c {
                '0'..='9' => self.state = AFDState::Accept(3),
                'e' | 'E' => self.state = AFDState::NonAccept(4),
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::Accept(6) => match c {
                '0'..='9' => self.state = AFDState::Accept(6),
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::Accept(9) => match c {
                '0'..='9' => self.state = AFDState::Accept(9),
                'a'..='z' | 'A'..='Z' => self.state = AFDState::Accept(9),
                '_' => self.state = AFDState::Accept(9),
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::Accept(13) => match c {
                '=' | '>' => {
                    self.done = true;
                    self.state = AFDState::Accept(15);
                }
                '-' => {
                    self.done = true;
                    self.state = AFDState::Accept(14);
                }
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::Accept(16) => match c {
                '=' => {
                    self.done = true;
                    self.state = AFDState::Accept(17);
                }
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::Accept(25) => match c {
                '0'..='9' => self.state = AFDState::Accept(25),
                c if is_valid(c) => self.end(),
                _ => self.err(0),
            },
            AFDState::NonAccept(2) => match c {
                '0'..='9' => self.state = AFDState::Accept(3),
                _ => self.err(2),
            },
            AFDState::NonAccept(4) => match c {
                '-' => self.state = AFDState::NonAccept(5),
                '+' => self.state = AFDState::NonAccept(24),
                '0'..='9' => self.state = AFDState::Accept(25),
                _ => self.err(3),
            },
            AFDState::NonAccept(5) => match c {
                '0'..='9' => self.state = AFDState::Accept(6),
                _ => self.err(5),
            },
            AFDState::NonAccept(7) => match c {
                '"' => {
                    self.done = true;
                    self.state = AFDState::Accept(8);
                }
                c if is_valid(c) => {
                    self.state = AFDState::NonAccept(7);
                }
                _ => self.err(0),
            },
            AFDState::NonAccept(10) => match c {
                '}' => {
                    self.state = AFDState::Initial;
                    self.action = Action::Clear;
                }
                c if is_valid(c) => self.state = AFDState::NonAccept(10),
                _ => self.err(0),
            },
            AFDState::NonAccept(24) => match c {
                '0'..='9' => self.state = AFDState::Accept(25),
                _ => self.err(4),
            },
            _ => (),
        }
    }

    fn err(&mut self, kind: u8) {
        self.done = true;
        self.state = AFDState::Error(kind);
        self.action = Action::ShowErrMessage;
    }

    fn end(&mut self) {
        self.done = true;
        self.action = Action::GoBack;
    }
}

fn is_valid(c: char) -> bool {
    match c {
        '0'..='9' => true,
        'a'..='z' => true,
        'A'..='Z' => true,
        ',' | '.' | ';' | ':' => true,
        '<' | '>' | '=' => true,
        '(' | ')' => true,
        '[' | ']' => true,
        '{' | '}' => true,
        '+' | '-' | '*' | '/' => true,
        '!' | '?' | '\\' => true,
        '"' | '\'' => true,
        '\n' | '\r' | ' ' => true,
        _ => false,
    }
}
