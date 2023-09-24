use crate::utils::command_parser;


pub fn test() {
    for s in [
        "asdf",
        "/abs -t -a -l 1",
        "/184 -t aof",
        "/newc -client_id 123 -name A -char_career base -team_id 1 -pos_r 11 -pos_c 2",
    ] {
        let res = command_parser::parse(s.to_string());
        match res {
            Ok(t) => {
                println!("{:#?}", t)
            },
            Err(e) => {
                println!("{}", e.to_string())
            }
        }
        
    }
}