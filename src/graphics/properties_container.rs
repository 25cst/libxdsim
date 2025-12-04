use std::{
    error::Error,
    fmt::{Display, Write},
};

use crate::graphics::{Menu, MenuInputValue};

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
}

#[derive(Debug)]
pub enum PropertiesContainerSetError {
    PropertyDoesNotExist,
    Invalid { reason: String },
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
