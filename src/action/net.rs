use reqwest::blocking::{ClientBuilder, Response, Client,multipart};
// use dotenv::dotenv;

use serde::{Deserialize,Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct UserForm{
    user: String,
    password: String
}

#[derive(Serialize,Deserialize, Debug)]
struct File{
    name: String,
    description: String,
    upload_date: String
}

#[derive(Serialize,Deserialize)]
struct NewFile{
    name: String,
    description: String
}

pub fn api_register(name: &str, password: &str) -> Result<Response,reqwest::Error>{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let data = UserForm {
        user: String::from(name),
        password: String::from(password)
    };
    let client = ClientBuilder::new().build().unwrap();
    client.post(format!("http://[{}]:{}/api/register",ip,port)).form(&data).send()
        // .expect("Error sending register request")

}

pub fn api_login(name: &str, password: &str) -> Result<Response,reqwest::Error>{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let data = UserForm {
        user: String::from(name),
        password: String::from(password)
    };
    let client = ClientBuilder::new().build().unwrap();
    client.post(format!("http://[{}]:{}/api/login",ip,port)).form(&data).send()

}

pub fn api_get() -> Response{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let client = ClientBuilder::new().build().unwrap();
    client.get(format!("http://[{}]:{}/api/get",ip,port)).send()
        .expect("Error sending get request")
}

pub fn api_download() -> Response{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let client = ClientBuilder::new().build().unwrap();
    client.get(format!("http://[{}]:{}/api/download",ip,port)).send()
        .expect("Error downloading a file")
}

pub fn api_upload(id_string: String, name: String, desc: String, file_path: &std::path::Path)
{
    let ip = dotenv::var("IP").unwrap();
    let port = dotenv::var("PORT").unwrap();
    let client = Client::new();
    let form = multipart::Form::new()
        .file("file",file_path).unwrap()
        .text("name",name)
        .text("description",desc)
        .text("session",id_string);

    let resp = client.post(format!("http://[{}]:{}/api/upload",ip,port))
        .multipart(form)
        .send()
        .unwrap();

    
}