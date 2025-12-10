pub trait Type {
    // Serialize Type into bytes
    fn serialize(&self) -> Box<[u8]>;
}
