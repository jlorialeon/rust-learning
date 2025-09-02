use std::env;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS:String = set_address();
    pub static ref PORT:u16 = set_port();
}

#[allow(dead_code)]
fn set_address() -> String{
    dotenv::dotenv().ok();
    return env::var("ADDRESS").unwrap();
}

#[allow(dead_code)]
fn set_port() -> u16{
    dotenv::dotenv().ok();
    return env::var("PORT").unwrap().parse::<u16>().unwrap();
}