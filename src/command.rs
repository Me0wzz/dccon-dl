use std::env;

pub async fn parse_param() -> Vec<String> {
    let param: Vec<String> = env::args().collect();
    let param = (&param[1..]).to_vec();
    println!("{param:?}");
    if param.is_empty() {
        panic!("give me url params");
    }
    param
}
