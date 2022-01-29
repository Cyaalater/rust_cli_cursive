use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use crate::views::login_view::login;


pub fn build() -> Dialog{
    Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Unsigned user"))
        .child(LinearLayout::horizontal()
            .child(Button::new("Login", login))
            .child(Button::new("Register", |s| {

            }))
            .child(Button::new("Show data", |s| {

            }))
        )
    )
}