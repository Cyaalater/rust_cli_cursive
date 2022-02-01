use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use crate::action::net::api_login;
use crate::views::print::cprint;

pub fn login(s: &mut Cursive)
{
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(LinearLayout::horizontal()
                    .child(EditView::new()
                        .with_name("username")
                        .fixed_width(10)
                    )
                    .child(
                        TextView::new("Username")
                    )
                )
                .child(LinearLayout::horizontal()
                    .child(EditView::new()
                        .with_name("password")
                        .fixed_width(10)
                    )
                    .child(
                        TextView::new("Password")
                    )

                )
                .child(Button::new("Login", |s| {
                    let user = s.call_on_name("username", |view: &mut EditView|{
                        view.get_content()
                    }).unwrap();
                    let password = s.call_on_name("password",|view: &mut EditView|{
                        view.get_content()
                    }).unwrap();
                    ok(s,&user,&password);

                }))
        )
    )

}
fn ok(s: &mut Cursive, username: &str, password: &str){
    // TODO: Insert into local memory or local storage
    // TODO: Optional: Fetch data like session
    let resp = api_login(username,password);
    cprint(s,resp.text().unwrap());
    // s.pop_layer();
}