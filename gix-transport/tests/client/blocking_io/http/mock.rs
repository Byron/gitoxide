use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr},
    time::Duration,
};

use bstr::ByteVec;
use gix_transport::{
    client::{http, TransportWithoutIO},
    Protocol,
};

use crate::fixture_bytes;

enum Command {
    ReadAndRespond(Vec<u8>),
}

enum CommandResult {
    ReadAndRespond(Vec<u8>),
}

pub struct Server {
    pub addr: SocketAddr,
    send_command: std::sync::mpsc::SyncSender<Command>,
    recv_result: std::sync::mpsc::Receiver<CommandResult>,
}

impl Server {
    pub fn new(fixture: Vec<u8>) -> Self {
        let ports = (15411..).take(10);
        let listener = std::net::TcpListener::bind(
            ports
                .map(|port| SocketAddr::from(([127, 0, 0, 1], port)))
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .expect("one of these ports to be free");
        let addr = listener.local_addr().expect("a local address");
        let (send_result, recv_result) = std::sync::mpsc::sync_channel(0);
        let (send_command, recv_commands) = std::sync::mpsc::sync_channel(0);
        std::thread::spawn(move || {
            for command in recv_commands {
                match command {
                    Command::ReadAndRespond(response) => {
                        let (mut stream, _) = listener.accept().expect("accept to always work");
                        stream
                            .set_read_timeout(Some(Duration::from_millis(50)))
                            .expect("timeout to always work");
                        stream
                            .set_write_timeout(Some(Duration::from_millis(50)))
                            .expect("timeout to always work");
                        let mut out = Vec::new();
                        stream.read_to_end(&mut out).ok();
                        stream.write_all(&response).expect("write to always work");
                        stream.flush().expect("flush to work");
                        stream.shutdown(Shutdown::Both).ok();
                        if send_result.send(CommandResult::ReadAndRespond(out)).is_err() {
                            break;
                        }
                    }
                }
            }
        });
        send_command
            .send(Command::ReadAndRespond(fixture))
            .expect("send to go through when thread is up");
        Server {
            addr,
            send_command,
            recv_result,
        }
    }

    pub fn next_read_and_respond_with(&self, fixture: Vec<u8>) {
        self.send_command
            .send(Command::ReadAndRespond(fixture))
            .expect("thread to be waiting");
    }

    pub fn received(&self) -> Vec<u8> {
        match self.recv_result.recv().expect("thread to be up") {
            CommandResult::ReadAndRespond(received) => received,
        }
    }

    pub fn received_as_string(&self) -> String {
        self.received().into_string().expect("utf8 only")
    }
}

pub fn serve_once(name: &str) -> Server {
    Server::new(fixture_bytes(name))
}

pub fn serve_and_connect(
    name: &str,
    path: &str,
    version: Protocol,
) -> Result<(Server, http::Transport<http::Impl>), crate::Error> {
    let server = serve_once(name);
    let url_str = format!(
        "http://{}:{}/{}",
        &server.addr.ip().to_string(),
        &server.addr.port(),
        path
    );
    let client = gix_transport::client::http::connect(url_str.as_str().try_into()?, version, false);
    assert_eq!(url_str, client.to_url().as_ref());
    Ok((server, client))
}
