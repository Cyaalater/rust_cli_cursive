// TODO: Fetch the get data from the server which isn't require key
use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

pub fn data(s: &mut Cursive)
{
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
            .child(
                TextView::new("Current files on the server")
            )
            .child(
                (
                    TextView::new("CONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\nCONTEXT VIEW\n")
                )
            )
            .child(
                Button::new("Close",|s| {
                    s.pop_layer();
                })
            )
        )
    )
}