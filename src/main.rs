mod command;
mod constant;
mod parse;
mod test;

use std::env;

use command::parse_param;
use test::dccon_req_test_1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //parse_param().await;
    dccon_req_test_1().await;
    Ok(())
}
