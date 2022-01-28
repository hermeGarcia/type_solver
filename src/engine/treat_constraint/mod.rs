use crate::solver_interface::*;
use super::*;

pub fn treat_constraint(
    constraint: Constraint,
    mut rest: ConstraintList,
) -> Result<(ConstraintList, bool), error::TyError> {
    let mut is_new = false;
    let operations = constraint_to_instructions::constraint_to_operations(constraint);
    for operation in operations {
        let solution = apply_operation::apply_operation(rest, operation)?;
        rest = solution.0;
        is_new = is_new || solution.1;
    }
    Ok((rest, is_new))
}
