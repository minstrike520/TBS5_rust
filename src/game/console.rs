use std::collections::{HashSet, HashMap};

#[derive(PartialEq, Eq, Hash)]
enum InputSession {
    Init,
    NewC,
    Start,
}

#[derive(Debug)]
pub struct IllegalOperation {
    reason: &'static str,
    detail: HashMap<&'static str, String>
}

pub struct Console {
    sessions: HashSet<InputSession>
}

impl Console {
    pub fn new() -> Self {
        let mut h = HashSet::new();
        h.insert(InputSession::Init);
        Console { sessions: h }
    }
    pub fn init(&mut self, cmd: HashMap<String, String>) -> Result<i32, IllegalOperation> {
        self.sessions.clear();
        self.sessions.insert(InputSession::NewC);
        self.sessions.insert(InputSession::Start);
        for parameter in ["client_id", "board_size"] {
            if cmd[parameter].clone().parse::<i32>().is_err() {
                let input_value = cmd["client_id"].clone();
                return Result::Err(IllegalOperation{
                    reason: "argument with inaprropriate input",
                    detail: HashMap::from([
                        ("argument", String::from(parameter)),
                        ("input value", input_value),
                        ("input should be", String::from("a number"))
                    ])
                })
            }
        }
        Result::Ok(0)
    }
}