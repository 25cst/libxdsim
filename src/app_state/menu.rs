pub struct Menu {
    items: Vec<MenuItem>,
}

pub struct MenuItem {
    tooltip: Option<String>,
    item_type: MenuItemVariant,
}

pub enum MenuItemVariant {
    Foldable { title: String, items: Vec<MenuItem> },
    Text { content: String },
    Input { id: String, input_type: MenuInput },
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
        placeholder: String,
    },
    Multiline {
        max_lines: Option<u32>,
        placeholder: String,
    },
    Dropdown {
        options: Vec<String>,
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
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
