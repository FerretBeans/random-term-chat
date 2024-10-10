//using this to send the http requests bcs i dont want to many words in a single file qwq
use http::*;


pub fn requwuest(ip_addr: &str) -> String {
    let mut req = Request::builder()
        .uri(ip_addr);
}