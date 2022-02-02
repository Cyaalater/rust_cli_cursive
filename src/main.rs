use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use dotenv::dotenv;
use std::env;

mod action;
mod views;

#[macro_use]
extern crate dotenv_codegen;
fn main(){
    let mut siv = cursive::default();
    dotenv().ok();
    if dotenv::var("IP").is_err() || dotenv::var("PORT").is_err()
    {
        siv.add_layer(views::config_view::setup())
    }else{
        siv.add_layer(views::splash::build());
    }
    siv.run();
    
}
// fn ok(s: &mut Cursive, key: &str){
//     s.add_layer(Dialog::text(key)
//         .title("works"))
// }

