use std::{error::Error, fmt::Display};

use crate::v0::app_state::{Menu, MenuInputValue};

pub trait PropertiesContainer {
    /// get the display layout of the menu
    fn get_menu(&self) -> Menu;

    /// get the value of an option using the ID of the option
    /// if the option does not exist, return None
    fn get_option(&self) -> Option<MenuInputValue>;

    /// set the value of an option using the ID of the option
    /// if the option does not exist or the provided the data is invalid
    fn set_option(
        &mut self,
        id: &str,
        value: MenuInputValue,
    ) -> Result<(), PropertiesContainerSetError>;

    /// serializes self to a byte array
    /// macro should also impl a top level function called
    /// deserialize_gate_option
    fn serialize(&self) -> Box<[u8]>;
}

#[derive(Debug)]
#[repr(C)]
pub enum PropertiesContainerSetError {
    PropertyDoesNotExist,
    Invalid { reason: Box<str> },
}

impl Display for PropertiesContainerSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PropertyDoesNotExist => f.write_str("The property does not exist."),
            Self::Invalid { reason } => {
                f.write_fmt(format_args!("The property value is invalid: {reason}"))
            }
        }
    }
}

impl Error for PropertiesContainerSetError {}
