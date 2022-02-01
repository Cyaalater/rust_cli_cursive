use reqwest::blocking::{ClientBuilder, Response};

use serde::{Deserialize,Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct UserForm{
    user: String,
    password: String
}

pub fn api_register(name: &str, password: &str) -> Response{
    let data = UserForm {
        user: String::from(name),
        password: String::from(password)
    };
    let client = ClientBuilder::new().build().unwrap();
    client.post("http://127.0.0.1:8000/api/register").form(&data).send()
        .expect("Error sending register request")

}

pub fn api_login(name: &str, password: &str) -> Response{
    let data = UserForm {
        user: String::from(name),
        password: String::from(password)
    };
    let client = ClientBuilder::new().build().unwrap();
    client.post("http://127.0.0.1:8000/api/login").form(&data).send()
        .expect("Error sending register request")

}