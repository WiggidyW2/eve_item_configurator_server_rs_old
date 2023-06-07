pub enum Kind {
    Read,
    Write,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
        }
    }

    pub fn from_bool(b: bool) -> Self {
        match b {
            false => Self::Read,
            true => Self::Write,
        }
    }
}

pub enum Scope {
    Items,
    Characters,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Items => "items",
            Self::Characters => "characters",
        }
    }

    pub fn from_bool(b: bool) -> Self {
        match b {
            false => Self::Items,
            true => Self::Characters,
        }
    }
}
