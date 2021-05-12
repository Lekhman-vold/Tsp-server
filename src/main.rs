use std::net::TcpListener;

fn main() {

    let host:&str = "127.0.0.1:8000";
    let listener = TcpListener::bind(host).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Server started {} >>>", host);
    }
}
