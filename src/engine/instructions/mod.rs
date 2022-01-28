use crate::solver_interface::{Constraint, Xtype};

pub type OperationList = std::collections::LinkedList<Operation>;

#[derive(Clone)]
pub enum Operation {
    Add(AddOp),
    Replace(ReplaceOp),
    Clash(ClashOp),
    Occurs(OccursOp),
}

#[derive(Clone)]
pub struct AddOp {
    pub is_new: bool,
    pub elem: Constraint,
}

#[derive(Clone)]
pub struct ReplaceOp {
    pub rhe: Xtype,
    pub lhe: Xtype,
}

#[derive(Clone)]
pub struct ClashOp {
    pub rhe: Xtype,
    pub lhe: Xtype,
}

#[derive(Clone)]
pub struct OccursOp {
    pub rhe: Xtype,
    pub lhe: Xtype,
}
