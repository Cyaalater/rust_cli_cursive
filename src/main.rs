use std::cell::RefCell;
use std::rc::Rc;
use dotenv::dotenv;
mod action;
mod views;




extern crate dotenv_codegen;
fn main(){

    // Session id inside an Rc<RfCell> in order to widely use around the modules
    let session_id = Rc::new(RefCell::new("".to_string()));
    
    // Cursive/tui Root
    let mut siv = cursive::default();

    // Dies if theme file not found
    if siv.load_theme_file("theme.toml").is_err()
    {
        println!("Theme file not found lol");
        return;
    }

    /*
        Expression worked from .env:
            if .env exists: Add splash screen layer
            else: Add config screen layer and later splash screen again
    */
    dotenv().ok();
    if dotenv::var("IP").is_err() || dotenv::var("PORT").is_err()
    {
        siv.add_layer(views::config_view::setup(session_id))
    }
    else
    {
        siv.add_layer(views::splash::build(session_id));
    }

    // Execute the tui
    siv.run();
    
}
// fn ok(s: &mut Cursive, key: &str){
//     s.add_layer(Dialog::text(key)
//         .title("works"))
// }

