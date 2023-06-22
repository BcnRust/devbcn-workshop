use std::fmt;

pub enum ButtonType {
    Primary,
    Secondary,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "text-slate-200 inline-flex items-center bg-cyan-700 border-0 py-1 px-3 focus:outline-none hover:bg-cyan-500 hover:text-teal-900 rounded mt-4 md:mt-0"),
            ButtonType::Secondary => write!(f, "text-slate-200 inline-flex items-center bg-rose-700 border-0 py-1 px-3 focus:outline-none hover:bg-rose-800 rounded mt-4 md:mt-0"),
        }
    }
}
