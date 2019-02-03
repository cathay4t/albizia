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

use std::fmt;
use std::result;
use toml;

pub type Result<T> = result::Result<T, AlbiziaError>;

#[derive(Debug)]
pub enum AlbiziaError {
    Bug(String),
}

impl ::std::error::Error for AlbiziaError {
    fn description(&self) -> &str {
        match *self {
            AlbiziaError::Bug(_) => "BUG",
        }
    }
}

impl fmt::Display for AlbiziaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                AlbiziaError::Bug(ref x) => x,
            }
        )
    }
}

impl From<::std::string::FromUtf8Error> for AlbiziaError {
    fn from(e: ::std::string::FromUtf8Error) -> Self {
        AlbiziaError::Bug(format!("Failed to convert from UTF-8 string: {}", e))
    }
}

impl From<::std::str::Utf8Error> for AlbiziaError {
    fn from(e: ::std::str::Utf8Error) -> Self {
        AlbiziaError::Bug(format!("Failed to convert from UTF-8 string: {}", e))
    }
}

impl From<::std::io::Error> for AlbiziaError {
    fn from(e: ::std::io::Error) -> Self {
        AlbiziaError::Bug(format!("IO Error: {}", e))
    }
}

impl From<::std::num::ParseIntError> for AlbiziaError {
    fn from(e: ::std::num::ParseIntError) -> Self {
        AlbiziaError::Bug(format!("ParseIntError: {}", e))
    }
}

impl From<::toml::de::Error> for AlbiziaError {
    fn from(e: ::toml::de::Error) -> Self {
        AlbiziaError::Bug(format!("Config parse error: {}", e))
    }
}
