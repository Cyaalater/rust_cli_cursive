use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use crate::views::data_view::data;
use crate::views::login_view::login;
use crate::views::register_view::register;
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
            .child(Button::new("Show data", send))
        )
    )
}

fn send(s: &mut Cursive)
{
    let data = RegisterForm {
        user: String::from("abc"),
        password: String::from("1234")
    };
    let client = reqwest::ClientBuilder::new()
        .build()
        .unwrap();
    let res = client.post(
        "127.0.0.1:8000/api/register"
    )
        .form(&data)
        .send();
}