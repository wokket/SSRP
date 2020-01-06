pub mod ssrp;

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        let dest = "127.0.0.1:1434";
        socket.connect(dest)?;

        let data = ssrp::get_instance_request("SQLEXPRESS");
        socket.send(&data)?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 1024];
        let amt = socket.recv(&mut buf)?;
        let data = &buf[..amt];

        println!("Received: {:?}", data);
    } // the socket is closed here
    Ok(())
}
