use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

pub fn login(s: &mut Cursive)
{
    s.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(EditView::new()
            .with_name("key")
            .fixed_width(10))
        .child(Button::new("Apply", |s| {
            let key = s.call_on_name("key",|view: &mut EditView| {
                view.get_content()
            }).unwrap();
            ok(s,&key);
        }))));

}
fn ok(s: &mut Cursive, key: &str){
    // TODO: Insert into local memory or local storage
    // TODO: Optional: Fetch data like session
    s.pop_layer();
}