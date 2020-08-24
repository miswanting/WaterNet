use std::net::UdpSocket;

fn main() {
    loop {
        let socket = UdpSocket::bind("0.0.0.0:6000").expect("Couldn't bind to address!");
        println!("Listening...");
        loop {
            let mut buf = [0; 16];
            let (amt, addr) = socket.recv_from(&mut buf).unwrap();
            let buf = &mut buf[..amt];
            println!("{:?}", buf);
            socket.send_to(buf, addr).unwrap();
        }
        println!("Closing...");
    }
}
