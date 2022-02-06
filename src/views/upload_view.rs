use cursive::traits::*;
use cursive::Cursive;
use cursive::{traits::Resizable, views::*};
use crate::views::select_path_view::select_path;
use crate::views::print::cprint;
pub fn upload(s: &mut Cursive)
{
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                                .child(
                                Button::new("Path",|s|{
                                    select_path(s);
                                })
                                )
                                .child(DummyView)
                                .child(
                                    TextView::new("Path-to-file")
                                )
                    )
                    .child(TextView::new("").with_name("file_path"))
                    .child(DummyView.fixed_height(1))
                    .child(
                        LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                .with_name("name")
                                .fixed_width(15)
                            )
                            .child(DummyView)
                            .child(
                                TextView::new("Name")
                            )
                    )
                    .child(DummyView.fixed_height(1))
                    .child(TextView::new("Description:"))
                    .child(
                        EditView::new()
                        .with_name("desc")
                        .fixed_width(25)
                    )
                    .child(DummyView)
                    .child(
                        LinearLayout::horizontal()
                        .child(
                            Button::new("Close",|s|{
                                s.pop_layer();
                            })
                        )
                        .child(
                            Button::new("Upload",|s|{
                                let user_path = s.call_on_name("file_path", |view: &mut TextView|{
                                    view.get_content()
                                }).unwrap();
                                let user_name = s.call_on_name("name", |view: &mut EditView|{
                                    view.get_content()
                                }).unwrap();
                                let user_desc = s.call_on_name("desc",|view: &mut EditView| {
                                    view.get_content()
                                }).unwrap();
                                if user_path.is_empty() || user_name.is_empty() || user_desc.is_empty(){
                                    cprint(s,format!("Please finish the form"));
                                    return;
                                }
                                s.pop_layer();
                            })
                        )
                    )
        )
    )
}