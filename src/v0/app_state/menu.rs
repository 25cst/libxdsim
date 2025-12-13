#[repr(C)]
pub struct Menu {
    pub items: Box<[MenuItem]>,
}

#[repr(C)]
pub struct MenuItem {
    pub tooltip: Option<Box<str>>,
    pub item_type: MenuItemVariant,
}

#[repr(C)]
pub enum MenuItemVariant {
    Foldable {
        title: Box<str>,
        items: Box<[MenuItem]>,
    },
    Text {
        content: Box<str>,
    },
    Input {
        id: Box<str>,
        input_type: MenuInput,
    },
}

#[repr(C)]
pub enum MenuInput {
    String {
        style: MenuInputStringStyle,
    },
    Integer {
        min: Option<i64>,
        max: Option<i64>,
        style: MenuInputIntegerStyle,
    },
    Float {
        min: Option<f64>,
        max: Option<f64>,
        style: MenuInputFloatStyle,
    },
    Boolean {
        style: MenuInputBooleanStyle,
    },
}

#[repr(C)]
pub enum MenuInputStringStyle {
    Inline {
        placeholder: Box<str>,
    },
    Multiline {
        max_lines: Option<u32>,
        placeholder: Box<str>,
    },
    Dropdown {
        options: Box<[Box<str>]>,
    },
}

#[repr(C)]
pub enum MenuInputIntegerStyle {
    Slider,
    TextBox,
    SliderAndTextBox,
}

#[repr(C)]
pub enum MenuInputFloatStyle {
    Slider,
    TextBox,
    SliderAndTextBox,
}

#[repr(C)]
pub enum MenuInputBooleanStyle {
    CheckBox,
}

#[repr(C)]
pub enum MenuInputValue {
    String(Box<str>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
