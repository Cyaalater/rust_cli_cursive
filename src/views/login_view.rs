use std::cell::{Cell, RefCell};
use std::rc::Rc;
use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use crate::action::net::api_login;
use crate::views::print::cprint;
use serde::{Deserialize,Serialize};
#[derive(Serialize,Deserialize)]
struct LoginResult {
    success: bool,
    session: String
}


pub fn login(s: &mut Cursive, session_id: Rc<RefCell<String>>)
{
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(LinearLayout::horizontal()
                    .child(EditView::new()
                        .with_name("username")
                        .fixed_width(10)
                    )
                    .child(DummyView)
                    .child(
                        TextView::new("Username")
                    )
                )
                .child(LinearLayout::horizontal()
                    .child(EditView::new()
                        .secret()
                        .with_name("password")
                        .fixed_width(10)
                    )
                    .child(DummyView)
                    .child(
                        TextView::new("Password")
                    )

                )
                .child(Button::new("Login", move |s| {
                    let user = s.call_on_name("username", |view: &mut EditView|{
                        view.get_content()
                    }).unwrap();
                    let password = s.call_on_name("password",|view: &mut EditView|{
                        view.get_content()
                    }).unwrap();
                    ok(s,&user,&password,session_id.clone());

                }))
                .child(Button::new("Close", |s| {
                    s.pop_layer();
                }))
        )
    )

}
fn ok(s: &mut Cursive, username: &str, password: &str, session_id: Rc<RefCell<String>>){

    // TODO: Insert into local memory or local storage
    // TODO: Optional: Fetch data like session
    let resp = api_login(username,password);
    if resp.is_err(){
        cprint(s,format!("Failed to connect to the server"));
        return;
    }
    let unwrapped_resp = resp.unwrap();
    // let result_text = unwrapped_resp..text().unwrap();
    let result_struct: LoginResult = unwrapped_resp.json::<LoginResult>().unwrap();
    // cprint(s,result_text.to_string());
    session_id.replace(result_struct.session);
    // s.pop_layer();
}