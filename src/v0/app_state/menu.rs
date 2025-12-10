pub struct Menu {
    pub items: Box<[MenuItem]>,
}

pub struct MenuItem {
    pub tooltip: Option<Box<str>>,
    pub item_type: MenuItemVariant,
}

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

pub enum MenuInputIntegerStyle {
    Slider,
    TextBox,
    SliderAndTextBox,
}

pub enum MenuInputFloatStyle {
    Slider,
    TextBox,
    SliderAndTextBox,
}

pub enum MenuInputBooleanStyle {
    CheckBox,
}

pub enum MenuInputValue {
    String(Box<str>),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
