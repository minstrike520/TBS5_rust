pub fn test_init() {
    use crate::game::console::Console as Console;
    use std::collections::HashMap;

    let init_data = HashMap::from([
        (String::from("client_id"), String::from("1A234")),
        (String::from("board_size"), String::from("1"))
    ]);


    let mut cons = Console::new();
    match cons.init(init_data) {
        Ok(T) => {
            println!("ok");
        },
        Err(E) => {
            println!("{:#?}", E)
        }
    }
}