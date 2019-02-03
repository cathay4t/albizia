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
//
use super::error::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

static CONFIG_PATH: &str = "/etc/albizia/albizia.toml";
const ABZD_TCP_PORT: u32 = 9909;
static ABZD_IP_ADDR: &str = "::";

#[derive(Clone, Deserialize)]
pub struct AlbiziaConf {
    #[serde(default)]
    pub port: u32,
    #[serde(default)]
    pub address: String,
    pub nodes: Vec<String>,
}

impl std::default::Default for AlbiziaConf {
    fn default() -> Self {
        AlbiziaConf {
            port: ABZD_TCP_PORT,
            address: ABZD_IP_ADDR.to_string(),
            nodes: Vec::new(),
        }
    }
}

pub fn load_conf() -> Result<AlbiziaConf> {
    let mut fd = File::open(CONFIG_PATH)?;
    let mut contents = String::new();
    fd.read_to_string(&mut contents)?;
    let conf: AlbiziaConf = toml::from_str(&contents)?;
    Ok(conf)
}
