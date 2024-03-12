use std::net::{TcpStream, UdpSocket};

pub enum NetworkMode {
    Udp,
    Tcp,
    UdpMulticast,
    TcpMulticast,
}

pub enum NetworkSender {
    Udp(UdpSocket),
    Tcp(TcpStream),
}
