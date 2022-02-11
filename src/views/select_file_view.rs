use cursive::traits::*;
use cursive::{views::*, traits::Resizable};
use cursive::Cursive;
use cursive::theme::{ColorStyle,Color,BaseColor};
use crate::views::download_view::StandardFile;


pub fn select_file(s: &mut Cursive, file_info: &StandardFile)
{
    let name = &file_info.name;
    let desc = &file_info.description;
    let date = &file_info.date;
    let name_view = TextView::new(name);
    let desc_view = TextView::new(desc);
    let date_view = TextView::new(format!("Upload date: {}",date))
        .style(ColorStyle::new(Color::Dark(BaseColor::Blue),ColorStyle::secondary().back));
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
            .child(TextView::new("NAME:"))
            .child(name_view)
            .child(DummyView)
            .child(TextView::new("DESCRIPTION:").scrollable())
            .child(desc_view)
            .child(DummyView.fixed_height(10))
            .child(date_view)
        )
        .button("Close", |s| {s.pop_layer();})
        .button("Download",|s| {s.pop_layer();})
        .title("File information")
        .fixed_width(40)
    )
}