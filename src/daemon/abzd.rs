// Copyright (C) 2019 Gris Ge
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// Author: Gris Ge <cnfourt@gmail.com>

#[macro_use]
extern crate serde_derive;
extern crate pnet;
extern crate toml;

mod conf;
mod error;
mod net;
mod ipc;
mod comm;

use conf::*;
use error::*;
use net::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

enum IpcData {
    ThreadAllReady,
    CommunicatorThreadReady,
    CommanderThreadReady,
    DoctorThreadReady,
    PrincipleThreadReady,
    UpdatePeerNodes(Vec<String>),
    StartPrinciple,
}

const IPC_BUFF_SIZE: usize = 8192;
const IPC_HDR_LEN: usize = 10; // length of u32 max string.

fn handle_client(mut stream: TcpStream) -> Result<String> {
    let mut msg_buff = [0u8; IPC_HDR_LEN];
    stream.read_exact(&mut msg_buff)?;
    let msg_len = str::from_utf8(&msg_buff)?.parse::<usize>()?;
    let mut msg = Vec::with_capacity(msg_len);
    let mut got: usize = 0;
    let mut msg_buff = [0u8; IPC_BUFF_SIZE];
    while got < msg_len {
        let cur_got = stream.read(&mut msg_buff)?;
        msg.extend_from_slice(&msg_buff[0..cur_got]);
        got += cur_got;
    }
    let msg = String::from_utf8(msg)?;
    Ok(msg)
}

fn tcp_recv_thread(conf: &AlbiziaConf) {
    let listener =
        TcpListener::bind(format!("{}:{}", conf.address, conf.port)).unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("{}", handle_client(stream.unwrap()).unwrap());
    }
}

fn tcp_send_thread(collector_recv: &Receiver<String>, peer_nodes: &[String]) {
    println!("peer_nodes: {:?}", peer_nodes);
}

fn main() {
    let conf = load_conf().expect("Failed to read config");
    let peer_nodes = get_peer_node_ips(&conf.nodes);
    let (out_msg_send, out_msg_recv) = mpsc::channel();

    thread::Builder::new()
        .name("tcp_recv_thread".into())
        .spawn(move || tcp_recv_thread(&conf))
        .expect("Failed to start 'tcp_recv_thread' thread");

    thread::Builder::new()
        .name("tcp_send_thread".into())
        .spawn(move || tcp_send_thread(&out_msg_recv, &peer_nodes))
        .expect("Failed to start 'tcp_send' thread");

    loop {
        thread::sleep(::std::time::Duration::from_secs(30));
    }
}
