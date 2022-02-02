use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use crate::views::data_view::data;
use crate::views::login_view::login;
use crate::views::register_view::register;
use crate::views::download_view::download;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RegisterForm{
    user: String,
    password: String
}

pub fn build() -> Dialog{
    Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Unsigned user"))
        .child(LinearLayout::horizontal()
            .child(Button::new("Login", login))
            .child(Button::new("Register", register))
            .child(Button::new("Show data", data))
            .child(Button::new("DEMO",download))
        )
    )
}
