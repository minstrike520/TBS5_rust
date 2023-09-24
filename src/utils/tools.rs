pub fn spaces(amount: i32) -> String {
    let mut s = String::new();
    for _i in 1..=amount {
        s = format!("{}{}", s, " ");
    };
    s
}