mod instructions;
mod apply_operation;
mod constraint_to_instructions;
mod treat_constraint;
mod type_fn;

use crate::solver_interface::*;

pub fn run_inference(mut constraints: ConstraintList) -> Result<ConstraintList, error::TyError> {
    let mut did_change = true;
    while did_change {
        did_change = false;
        let work_buff = constraints.len();
        let mut index = 0;
        while index < work_buff {
            let constraint = constraints.pop_front().unwrap();
            let result = treat_constraint::treat_constraint(constraint, constraints)?;
            did_change = did_change || result.1;
            constraints = result.0;
            index += 1;
        }
    }
    Ok(constraints)
}
