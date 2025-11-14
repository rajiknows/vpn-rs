use std::net::SocketAddr;

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};

pub struct UdpTransport {
    socket: UdpSocket,
    poll: Poll,
    events: Events,
    peer: SocketAddr,
}

impl UdpTransport {
    pub fn new(local: SocketAddr, peer: SocketAddr) -> std::io::Result<Self> {
        let mut socket = UdpSocket::bind(local)?;
        socket.connect(peer)?;
        let poll = Poll::new()?;
        let events = Events::with_capacity(1024);
        poll.registry()
            .register(&mut socket, Token(0), Interest::READABLE)?;
        Ok(Self {
            socket,
            poll,
            events,
            peer,
        })
    }

    pub fn send(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.socket.send(data)
    }

    pub fn recv(&mut self, data: &mut [u8]) -> std::io::Result<Option<usize>> {
        self.poll
            .poll(&mut self.events, Some(std::time::Duration::from_millis(10)))?;
        for event in self.events.iter() {
            if event.is_readable() {
                return match self.socket.recv(data) {
                    Ok(n) => Ok(Some(n)),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(None),
                    Err(e) => Err(e),
                };
            }
        }
        Ok(None)
    }
}
