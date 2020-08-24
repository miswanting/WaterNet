use std::io;
use std::net::UdpSocket;
fn main() {
    let socket = UdpSocket::bind("0.0.0.0:22").expect("Couldn't bind to address!");
    let mut buf = String::new();
    let amt = io::stdin().read_line(&mut buf).unwrap();
    let buf: Vec<_> = &mut buf[..amt].split_whitespace().collect().as_vec();
    println!("{}", buf == "connect");
    if buf == "connect" {
        println!("Connecting...");
        socket
            .connect("127.0.0.1:6000")
            .expect("Connect function failed!");
        socket.send(&[0, 1, 2]).expect("Couldn't send message");
        let mut buf = [0; 16];
        let amt = socket.recv(&mut buf).unwrap();
        let buf = &mut buf[..amt];
        println!("{:?}", buf)
    }
}
