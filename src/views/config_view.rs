use std::cell::RefCell;
use std::rc::Rc;
use cursive::traits::*;
use cursive::Cursive;
use cursive::{traits::Resizable, views::*};
use crate::views::splash;


pub fn setup(session_id: Rc<RefCell<String>>) -> Dialog {
    let mut ipv_radio: RadioGroup<i32> = RadioGroup::new();

    Dialog::around(
        LinearLayout::vertical()
            .child(TextView::new("Enter server's location"))
            .child(
                LinearLayout::horizontal()
                    .child(EditView::new().with_name("IP").fixed_width(15))
                    .child(TextView::new("IP")),
            )
            .child(DummyView)
            .child(
                LinearLayout::horizontal()
                    .child(EditView::new().with_name("PORT").fixed_width(10))
                    .child(TextView::new("PORT")),
            )
            .child(DummyView)
            .child(LinearLayout::horizontal()
                .child(ipv_radio.button(4, "IPv4"))
                .child(ipv_radio.button(6, "IPv6"))
        
            )
            .child(DummyView)
            .child(Button::new("Apply", move |s| {
                let ip = s
                    .call_on_name("IP", |view: &mut EditView| view.get_content())
                    .unwrap();
                let port = s
                    .call_on_name("PORT", |view: &mut EditView| view.get_content())
                    .unwrap();
                ok(s, &ip, &port,session_id.clone(),ipv_radio.selection());
            }))
    )
}

fn ok(s: &mut Cursive, ip: &str, port: &str,session_id: Rc<RefCell<String>>, ipv_type: Rc<i32>) {
    
    std::fs::write("./.env", format!("IP={}\nPORT={}\nTYPE={}", ip, port,ipv_type))
        .expect("Failed writing .env file");
    s.pop_layer();
    s.add_layer(splash::build(session_id));
}
