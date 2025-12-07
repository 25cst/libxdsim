use crate::{
    app_state::PropertiesContainer,
    component::{connection_definition::ConnectionDefinition, connection_path::ConnectionPath},
    graphics::Graphic,
};

/// A connection that can carry a value
/// It may (or not) necessarily be connected to anything
pub trait Connection {
    /// Produce a graphic of the connection.
    /// Request is a reference because it doesn't really make sense
    /// for the draw function to take ownership of anything.
    fn draw(&self, request: &ConnectionDrawRequest) -> Graphic;

    /// Returns connection definition. This may depend on options so takes `&self`.
    fn definition(&self) -> ConnectionDefinition;

    /// Get the property container
    /// this is to be implemented by macro
    fn properties_container(&self) -> &dyn PropertiesContainer;

    /// Get the property container (mutable)
    /// this is to be implemented by macro
    fn properties_container_mut(&mut self) -> &mut dyn PropertiesContainer;

        /// Serialize connection into bytes
    /// macro will remove the &self from the argument list
    fn serialize(&self, data: *const u8) -> Box<[u8]>;

    /// Implemented by macro
    /// serializes the data
    fn serialize_data(&self, data: *const u8) -> Box<[u8]>;

    /// Implemented by macro
    /// deserializes the data
    fn deserialize_data(&self, bytes: Box<[u8]>) -> *const u8;

    /// Call free() on the data
    fn drop_data(&self, data: *const u8);
}

/// Details of a request to draw a `Connection`
pub struct ConnectionDrawRequest {
    /// Path the connection takes
    pub path: ConnectionPath,
    /// Current value in the connection
    pub data: *const (),
}
