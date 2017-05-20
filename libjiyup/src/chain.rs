#[derive(Serialize, Deserialize)]
pub struct Block {
    version: u16,
    parent: Address,
    timestamp: u64, // UNIX
    author: Address,
    artifacts: Vec<Address>
}

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    version: u16,
    parent: Address,
    timestamp: u64, // UNIX
    author: Address,
    difficulty: u64, // FIXME Define this better.
    blocks: Vec<Address>
}

// FIXME Make this struct deserialize more compactly, like to just a hex string.
#[derive(Serialize, Deserialize)]
pub struct Address {
    id: [u8; 32] // FIXME Make this 32 configuratble based on which hash algo we choose.
}
