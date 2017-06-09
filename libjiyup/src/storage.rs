#![allow(unused_imports, dead_code, unused_variables)]

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fmt;

use chain::*;
use artifact::*;

use serde::*;
use serde::de::*;
use serde::ser::*;

pub trait BlockchainElement {
    fn encode(&self) -> &[u8];
    fn decode(bytes: &[u8]) -> Self;
}

pub struct Address {
    id: [u8; 32] // FIXME Make this 32 configuratble based on which hash algo we choose.
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.to_string().as_str())
    }
}

struct AddrVisitor;
impl<'de> Visitor<'de> for AddrVisitor {

    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string of length 32")
    }

}

impl<'de> Deserialize<'de> for Address {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        unimplemented!();
    }

}

impl Address {

    pub fn from_str(hex: &str) -> Address {
        unimplemented!()
    }

    pub fn to_string(&self) -> String {

        let mut hex = String::with_capacity(64);

        for b in self.id.into_iter() {

            let conv = |n| match n {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                _ => 'f' // ???
            };

            hex.push(conv((b & 0xf0) >> 4));
            hex.push(conv(b & 0x0f));

        }

        assert_eq!(hex.len(), 64);
        hex.to_owned()

    }

}

pub trait Entity {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> &[u8];
}

pub trait EntitySource<E> where E: Entity {
    fn load(&self, addr: Address) -> Result<Option<E>, ()>;
    fn publish(&self, ent: E);
}

pub struct LocalStorage {
    root: PathBuf
}

impl LocalStorage {

    fn new(path: PathBuf) -> Result<LocalStorage, ()> {
        if path.is_dir() {
            Ok(LocalStorage { root: path })
        } else {
            Err(())
        }
    }

}

impl<E> EntitySource<E> for LocalStorage where E: Entity {

    fn load(&self, addr: Address) -> Result<Option<E>, ()> {

        let mut p = self.root.to_owned();
        let addr_str = addr.to_string();
        p.push(&addr_str[..4]);
        p.push(&addr_str[4..]);

        if p.exists() && p.is_file() {

            let mut f = File::open(p).unwrap();
            let mut buf = Vec::new();
            match f.read_to_end(buf.as_mut()) {
                Ok(v) => Ok(Some(E::from_bytes(buf.as_slice()))),
                Err(_) => Err(())
            }

        } else {
            Ok(None)
        }

    }

    fn publish(&self, ent: E) {
        unimplemented!();
    }

}
