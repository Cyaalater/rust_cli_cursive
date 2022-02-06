use cursive::{views::*, traits::Resizable};
use crate::views::data_view::data;
use crate::views::login_view::login;
use crate::views::register_view::register;
use crate::views::download_view::download;
use crate::views::upload_view::upload;
use crate::views::select_path_view::select_path;
// use crate::views::select_file_view::select_file;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RegisterForm{
    user: String,
    password: String
}

pub fn build() -> Dialog{
    Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Unsigned user").center())
        .child(DummyView.fixed_height(2))
        .child(LinearLayout::horizontal()
            .child(Button::new("Login", login))
            .child(DummyView)
            .child(Button::new("Register", register))
            .child(DummyView)
            .child(Button::new("Show data", data))
            .child(DummyView)
            .child(Button::new("DEMO",upload))
        )
    )
}
