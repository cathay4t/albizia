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

use error::*;
use ipc::AbzIpcData;
use std::sync::mpsc::Sender;

pub struct AbzComm {
    socks: Vec<std::net::TcpStream>,
    send_to_comm: Sender,
}

impl AbzComm {
    pub fn start(send_to_node_mgr: &mut Sender) -> Result<AbzComm> {
        // Start a thread to select all peer nodes sockets and forward all
        // received data to AbzNodeMgr
        Ok(())
    }
    pub fn send(&self, node: Option<&str>, data: &AbzIpcData) -> Result<()> {
        Ok(())
    }
    pub fn add_node(&self, node: &str) -> Result<()> {
        Ok(())
    }
    pub fn del_node(&self, node: &str) -> Result<()> {
        Ok(())
    }
}
