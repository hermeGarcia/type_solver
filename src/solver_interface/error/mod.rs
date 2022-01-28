use super::ir::Xtype;
use std::fmt::Formatter;


pub enum TyError {
    Clash(ClashErr),
    Occurs(OccursErr),
}

pub struct ClashErr {
    pub rhe: Xtype,
    pub lhe: Xtype,
}

pub struct OccursErr {
    pub rhe: Xtype,
    pub lhe: Xtype,
}


impl std::error::Error for TyError {}

impl std::fmt::Display for TyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TyError::Clash(err) => {
                write!(f, "INCOMPATIBLE CONSTRAINT: ")?;
                err.lhe.fmt(f)?;
                write!(f, " = ")?;
                err.rhe.fmt(f)?;
                write!(f, "<!>")
            }
            TyError::Occurs(err) => {
                write!(f, "RECURSIVE CONSTRAINT: ")?;
                err.lhe.fmt(f)?;
                write!(f, " = ")?;
                err.rhe.fmt(f)?;
                write!(f, "<!>")
            }
        }
    }
}

impl std::fmt::Debug for TyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
