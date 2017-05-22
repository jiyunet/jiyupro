pub trait BlockchainElement {
    fn encode(&self) -> &[u8];
    fn decode(bytes: &[u8]) -> Self;
}

// FIXME Make this struct deserialize more compactly, like to just a hex string.
#[derive(Serialize, Deserialize)]
pub struct Address {
    id: [u8; 32] // FIXME Make this 32 configuratble based on which hash algo we choose.
}

impl Address {

    pub fn to_str(&self) -> String {

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
