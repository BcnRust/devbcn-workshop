use std::fmt;

pub enum ButtonType {
    Primary,
    Secondary,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "bg-blue-700 hover:bg-blue-800 active:bg-blue-900"),
            ButtonType::Secondary => write!(f, "bg-rose-700 hover:bg-rose-800 active:bg-rose-900"),
        }
    }
}
