#![allow(dead_code)]

pub struct Address {
    hash: [u8; 32] // FIXME Make 32 a constant when we can because rust doesn't like that for some reason.
}

impl Address {

    fn from_data(data: &[u8]) -> Address{

        // TODO Make it SHA-256 the data.

        Address {
            hash: [data[0], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        }

    }

}
