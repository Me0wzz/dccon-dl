use std::{collections::VecDeque, env};

pub async fn parse_param() -> Option<Vec<String>> {
    let param: Vec<String> = env::args().collect();
    let param = (&param[1..]).to_vec();
    println!("{param:?}");
    if param.is_empty() {
        None
    } else {
        Some(param)
    }
}
