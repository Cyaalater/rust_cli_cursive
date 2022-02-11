use std::path::PathBuf;
// use std::slice::SliceIndex;

// use cursive::traits::*;
use cursive::Cursive;
use cursive::views::*;

// use super::print::cprint;

pub fn select_path(s: &mut Cursive)
{
    let mut files: Vec<PathBuf> = Vec::new();
    let director = std::fs::read_dir(".").unwrap();
    let mut paths = SelectView::new()
        .on_submit(|s,i: &String |{
            travel_path(s,i)
        });
    for file in director.into_iter()
    {
        if file.is_ok()
        {
            let path = file.unwrap().path();
            files.push(path.to_owned());
            paths.add_item(path.to_owned().file_name().unwrap().to_str().unwrap(),format!("{}",path.to_str().unwrap()));
        }
    }
    
    s.add_layer(
        Dialog::around(paths)
        .button("Close",|s| {s.pop_layer();})
    )    
}

fn travel_path(s: &mut Cursive, path: &String)
{
    if std::fs::read_dir(path).is_err()
    {
        s.call_on_name("file_path", |view: &mut TextView|{
            view.set_content(path);
        }).unwrap();
        s.pop_layer();
        return;
    }
    // let mut files: Vec<PathBuf> = Vec::new();
    let director = std::fs::read_dir(path).unwrap();
    let mut paths : Vec<PathBuf> = Vec::new();
    let mut names: SelectView<String> = SelectView::new()
        .on_submit(|s,i: &String|{
            travel_path(s,i);
        });
    for file in director.into_iter()
    {
        if file.is_ok()
        {
            let path = file.unwrap().path();
            paths.push(path.to_owned());
            names.add_item(path.to_owned().file_name().unwrap().to_str().unwrap(),format!("{}",path.to_str().unwrap()));
        }
    };
    s.pop_layer();
    s.add_layer(
        Dialog::around(
            names
        ).button("Close", |s|{s.pop_layer();})
    )    

}

