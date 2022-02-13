use std::cell::RefCell;
use std::rc::Rc;
use cursive::Cursive;
use cursive::{views::*, traits::{Resizable, Nameable}};
// use crate::views::data_view::data;
use crate::views::login_view::login;
use crate::views::register_view::register;
use crate::views::download_view::download;
use crate::views::upload_view::upload;
// use crate::views::select_path_view::select_path;
// use crate::views::select_file_view::select_file;
use serde::{Deserialize,Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RegisterForm{
    user: String,
    password: String
}

pub fn build(session_id: Rc<RefCell<String>>) -> NamedView<Dialog>{
    let session_id1 = session_id.clone();
    Dialog::around(LinearLayout::vertical()
        .child(TextView::new("Unsigned user").center().with_name("splash_text"))
        .child(DummyView.fixed_height(2))
    )
    .button("Login", move |s|{login(s,session_id1.clone())})
    .button("Register", register)
    .with_name("splash")
}
pub fn logon_splash(s: &mut Cursive,session_id: Rc<RefCell<String>>)
{
    s.call_on_name("splash",|view: &mut Dialog| {
        let session_id1 = session_id.clone();
        view.clear_buttons();
        view.add_button("Show data", move |s| {download(s,session_id.clone())});
        view.add_button("DEMO", move |s| {upload(s, session_id1.clone())});
    });
}