use std::collections::HashMap;
use crate::utils;

const COMMAND_INDICATOR: char = '/';

const PARAMETER_INDICATOR: char = '-';

#[derive(Debug)]
pub struct IllegalInstruction {
    err_msg: String,
    instruction_content: String,
    position: i32
}


impl IllegalInstruction {
    pub fn get_description(&self) -> String {
        format!(
            "{}\n...{}\n...{}^ ",
            self.err_msg, 
            self.instruction_content, 
            utils::tools::spaces(self.position)
        )
    }
}

impl ToString for IllegalInstruction {
    fn to_string(&self) -> String {
        self.get_description()
    }
}

pub fn parse(s: String) -> Result<HashMap<String, Option<String>>, IllegalInstruction> {
    // legal char range: 48u8..=57 65u8..=90 97u8..=122 (x)
    // iterating way looks not suitable for static type which requires const fn
    let parts = s.split(" ")
        .map(|x| String::from(x))
        .collect::<Vec<String>>();
    if parts[0].chars().nth(0).unwrap() != COMMAND_INDICATOR {
        return Result::Err(IllegalInstruction {
            err_msg: format!("command starts without `{}`", COMMAND_INDICATOR),
            instruction_content: s,
            position: parts.join("").len() as i32
        })
    }
    let mut cmd_dict = HashMap::from([
        ("command".to_string(), Some(parts[0].clone().split("/").collect()))
    ]);
    for i in 1..parts.len() {
        if parts[i].chars().nth(0).unwrap() == PARAMETER_INDICATOR {
            let mut value: Option<String> = None;
            match parts[i+1].chars().nth(0) {
                Some(first_char) => {
                    if first_char != PARAMETER_INDICATOR {
                        value = Some(parts[i+1].clone())
                    }
                }
                None => ()
            }
            cmd_dict.insert(
                parts[i].clone().split("-").collect(), 
                value
            );
        } 
        else if parts[i].chars().nth(0).unwrap().is_alphanumeric() != true {
            return Err(IllegalInstruction { 
                err_msg: format!("invalid parameter value: first character must be alphanumeric or `{}`", PARAMETER_INDICATOR), 
                instruction_content: s, 
                position: parts[0..i].join(" ").len() as i32
            })
        }
    }
    Result::Ok(cmd_dict)
}