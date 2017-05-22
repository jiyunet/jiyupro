pub trait BlockchainElement {
    fn encode(&self) -> &[u8];
    fn decode(bytes: &[u8]) -> Self;
}
