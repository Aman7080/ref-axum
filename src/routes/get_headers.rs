use axum::http::HeaderMap;

pub async fn get_header(headers:HeaderMap) -> String{
    print!("{:#?}", headers);
    let token = headers.get("x-auth-token").unwrap().to_str().unwrap().to_owned();
    token

}