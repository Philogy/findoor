#[derive(Clone, Copy)]
pub enum NumFormat {
    Giga,
    Mega,
    Kilo,
    Uni,
}

impl NumFormat {
    pub fn factor(&self) -> f64 {
        match self {
            Self::Giga => 1_000_000_000.0,
            Self::Mega => 1_000_000.0,
            Self::Kilo => 1_000.0,
            Self::Uni => 1.0,
        }
    }

    pub fn letter(&self) -> &'static str {
        match self {
            Self::Giga => "B",
            Self::Mega => "M",
            Self::Kilo => "K",
            Self::Uni => "",
        }
    }
}
