use std::net::TcpListner;

fn main() {
    let listener = TcpListner("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connected");
    }
}
