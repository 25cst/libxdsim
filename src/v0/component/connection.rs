use crate::{
    defs::ConnectionDefinition,
    v0::{
        app_state::PropertiesContainer,
        component::{Type, connection_path::ConnectionPath},
        graphics::Graphic,
    },
};

/// A connection that can carry a value
/// It may (or not) necessarily be connected to anything
pub trait Connection {
    /// Produce a graphic of the connection.
    /// Request is a reference because it doesn't really make sense
    /// for the draw function to take ownership of anything.
    fn draw(&self, request: &ConnectionDrawRequest) -> Graphic;

    /// Returns connection definition. This may depend on options so takes `&self`.
    fn definition(&self) -> Box<dyn ConnectionDefinition>;

    /// Get the property container
    /// this is to be implemented by macro
    fn properties_container(&self) -> &dyn PropertiesContainer;

    /// Get the property container (mutable)
    /// this is to be implemented by macro
    fn properties_container_mut(&mut self) -> &mut dyn PropertiesContainer;

    /// Serialize connection into bytes
    /// macro will remove the &self from the argument list
    fn serialize(&self) -> Box<[u8]>;
}

/// Details of a request to draw a `Connection`
#[repr(C)]
pub struct ConnectionDrawRequest<'a> {
    /// Path the connection takes
    pub path: ConnectionPath,
    /// Current value in the connection
    pub data: &'a dyn Type,
}

impl<'a> ConnectionDrawRequest<'a> {
    pub fn get_data<T>(&self) -> &'a T {
        let ptr: *const dyn Type = &*self.data;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }
}
