use crate::solver_interface::ir::{ConstantID, VarID, ConstraintList, Constraint};
use crate::solver_interface::error::TyError;
use crate::engine;

#[derive(Clone, Default)]
pub struct BlackBoard {
    pub(crate) fresh_constant: ConstantID,
    pub(crate) fresh_var: VarID,
    pub(crate) constraints: ConstraintList,
}

impl BlackBoard {
    pub fn new() -> BlackBoard {
        BlackBoard::default()
    }

    pub fn new_constant(&mut self) -> ConstantID {
        self.fresh_constant.fresh_id()
    }

    pub fn new_var(&mut self) -> VarID {
        self.fresh_var.fresh_id()
    }

    pub fn add_constraint(&mut self, c: Constraint) {
        self.constraints.push_back(c);
    }

    pub fn rm_constraint(&mut self, index: usize) {
        let mut list = std::mem::take(&mut self.constraints);
        let mut head = list.split_off(index);
        let mut tail = list;
        head.pop_front();
        head.append(&mut tail);
        self.constraints = head;
    }

    pub fn infer(&mut self) -> Result<(), TyError> {
        let constraints = std::mem::take(&mut self.constraints);
        self.constraints = engine::run_inference(constraints)?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.constraints.clear();
    }

    pub fn get_constraints(&self) -> &ConstraintList {
        &self.constraints
    }

    pub fn set_constraints(&mut self, list: ConstraintList) {
        self.constraints = list;
    }
}
