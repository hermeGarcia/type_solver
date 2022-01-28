#[allow(clippy::all)]
mod parser;

use std::error::Error;
use std::fmt::{Display, Formatter, Debug};
pub trait Parse: Sized {
    fn parse(data: &str) -> Result<Self, ParseErr>;
}


pub struct ParseErr(String);

impl From<String> for ParseErr {
    fn from(x: String) -> Self {
        ParseErr(x)
    }
}

impl Error for ParseErr {}
impl Display for ParseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Debug for ParseErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseErr {{ msg: {} }}", self.0)
    }
}

pub type ConstraintList = std::collections::LinkedList<Constraint>;


#[derive(Clone)]
pub struct Constraint {
    pub(crate) lhe: Xtype,
    pub(crate) rhe: Xtype,
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.lhe.fmt(f)?;
        write!(f, " = ")?;
        self.rhe.fmt(f)
    }
}

impl From<&str> for Constraint  {
    fn from(data: &str) -> Self {
        parser::ConstraintParser::new().parse(data).unwrap()
    }
}

impl Parse for Constraint {
    fn parse(data: &str) -> Result<Self, ParseErr> {
        parser::ConstraintParser::new()
            .parse(data)
            .map_err(|x| ParseErr::from(x.to_string()))
    }
}

impl From<(Xtype, Xtype)> for Constraint {
    fn from((lhe, rhe): (Xtype, Xtype)) -> Self {
        Constraint { lhe, rhe }
    }
}

impl From<Constraint> for (Xtype, Xtype) {
    fn from(c: Constraint) -> Self {
        (c.lhe, c.rhe)
    }
}


#[derive(Clone)]
pub enum Xtype {
    Function(FunctionData),
    Var(VarData),
    Constant(ConstantData),
}

impl From<&str> for Xtype {
    fn from(data: &str) -> Self {
        parser::XtypeParser::new().parse(data).unwrap()
    }
}

#[derive(Default, Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ConstantID(usize);
impl ConstantID {
    pub fn fresh_id(&mut self) -> ConstantID {
        std::mem::replace(self, ConstantID(self.0 + 1))
    }
}

#[derive(Clone)]
pub struct ConstantData {
    pub name: ConstantID,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VarID(usize);
impl VarID {
    pub fn fresh_id(&mut self) -> VarID {
        std::mem::replace(self, VarID(self.0 + 1))
    }
}

#[derive(Clone)]
pub struct VarData {
    pub name: VarID,
}

#[derive(Clone)]
pub struct FunctionData {
    pub from: Box<Xtype>,
    pub to: Box<Xtype>,
}

impl std::fmt::Display for Xtype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Xtype::Constant(c) => {
                write!(f, "C({})", c.name.0)
            }
            Xtype::Var(v) => {
                write!(f, "V({})", v.name.0)
            }
            Xtype::Function(fun) => {
                fun.to.fmt(f)?;
                write!(f, " -> ")?;
                fun.from.fmt(f)
            }
        }
    }
}
