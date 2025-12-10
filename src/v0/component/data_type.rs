pub trait Type {
    // Serialize Type into bytes
    fn serialize(&self) -> Box<[u8]>;

    // Deserialize Type from bytes
    fn deserialize(bytes: Box<[u8]>) -> Self where Self: Sized;
}
