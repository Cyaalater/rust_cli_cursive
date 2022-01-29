use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

pub fn login(s: &mut Cursive)
{
    s.add_layer(LinearLayout::horizontal()
        .child(EditView::new()
            .with_name("key")
            .fixed_width(10))
        .child(Button::new("Apply", |s| {
            s.pop_layer();
        })))

}