// TODO: Will require username and maybe a password and then will return a unique id
// TODO: The server will create a low level user
use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

use crate::action::net::api_register;
use crate::views::print::cprint;

pub fn register(s: &mut Cursive){
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(EditView::new()
                            .with_name("username")
                            .fixed_width(10)
                        )
                        .child(DummyView)
                        .child(TextView::new("Username"))
                )
                .child(
                    LinearLayout::horizontal()
                        .child(EditView::new()
                            .secret()
                            .with_name("password")
                            .fixed_width(10)
                        )
                        .child(DummyView)
                        .child(TextView::new("Password"))
                )
                .child(
                    Button::new("Register",|s| {
                        let username = s.call_on_name("username",|view: &mut EditView| {
                            view.get_content()
                        }).unwrap();
                        let password = s.call_on_name("password", |view: &mut EditView| {
                            view.get_content()
                        }).unwrap();
                        ok(s,&username,&password);
                    })
                )
                .child(
                    Button::new("Close",|s| {
                        s.pop_layer();
                    })
                )
        )
    );
}


fn ok(s: &mut Cursive, name: &str, password: &str)
{
    let resp = api_register(name,password);
    if resp.is_err(){
        cprint(s,format!("Failed to connect to the server"));
        return;
    }
    cprint(s,resp.unwrap().text().unwrap());
}

// fn send_form(username: &str, password: &str) -> Result<(),reqwest::Error>
// {
//     let data = RegisterForm {
//         user: String::from(username),
//         password: String::from(password)
//     };
//     let client = reqwest::Client::new();
//     let res = client.post(
//         "http://127.0.0.1:8000/api/register"
//     )
//         .form(&data)
//         .send();
//     Ok(())
// }