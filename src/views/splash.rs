use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

pub fn build() -> Dialog{
    let input = LinearLayout::horizontal()
    .child(EditView::new().fixed_width(10).with_name("name"))
    .child(Button::new("Login",input_key));
    let primary = LinearLayout::vertical()
    .child(TextView::new("KEY: "))
    .child(input);
    Dialog::around(primary)
        .title("Cool cli view")

}

fn input_key(s: &mut Cursive){
    let ok = |s: &mut Cursive, key: &str| -> &str {
        return key;
    };
    let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();

    // println!("{}",key);
    s.add_layer(Dialog::around(TextView::new("HEY"))
    .button("close", |s| {
        s.pop_layer();
    })
    .title("EPIC")    
    );
}

// fn input_key(s: &mut Cursive){
//     let key = s.call_on_name("name", |view: &mut EditView| {
//         view.get_content()
//     }).unwrap();
//     s.add_layer(
//         Dialog::around(TextView::new(key.to_string()))
//                 .button("close", |f| {
//                     f.pop_layer();
//                 })
//             );
// }