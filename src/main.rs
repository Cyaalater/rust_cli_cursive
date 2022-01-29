use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

mod views;
fn main(){
    let mut siv = cursive::default();
    siv.add_layer(views::splash::build());
    siv.run();
}
// fn ok(s: &mut Cursive, key: &str){
//     s.add_layer(Dialog::text(key)
//         .title("works"))
// }