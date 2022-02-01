// TODO: Fetch the get data from the server which isn't require key
use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;

pub fn data(s: &mut Cursive)
{
    s.add_layer(
        Dialog::around(
            TextView::new("DATA CONTEXT")
        )
    )
}