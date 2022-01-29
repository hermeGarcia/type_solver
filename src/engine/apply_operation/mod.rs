use crate::solver_interface::*;
use crate::engine::instructions::*;
use crate::engine::type_fn;
use crate::solver_interface::error::*;

fn apply_replace(t: Xtype, op: &ReplaceOp) -> (Xtype, bool) {
    match t {
        t if type_fn::is_eq(&t, &op.lhe) => (op.rhe.clone(), true),
        Xtype::Function(mut f) => {
            let (to, effective_to) = apply_replace(*f.to, op);
            let (from, effective_from) = apply_replace(*f.from, op);
            f.to = Box::new(to);
            f.from = Box::new(from);
            (Xtype::Function(f), effective_to || effective_from)
        }
        _ => (t, false),
    }
}

pub fn apply_operation(
    mut constraints: ConstraintList,
    operation: Operation,
) -> Result<(ConstraintList, bool), TyError> {
    match operation {
        Operation::Add(AddOp { is_new, elem }) => {
            constraints.push_back(elem);
            Ok((constraints, is_new))
        }
        Operation::Replace(op) => {
            let mut new_constraints = ConstraintList::new();
            let mut effective = false;
            for Constraint { lhe, rhe } in constraints {
                let (lhe, effective_l) = apply_replace(lhe, &op);
                let (rhe, effective_r) = apply_replace(rhe, &op);
                effective = effective || effective_l || effective_r;
                new_constraints.push_back(Constraint::from((lhe, rhe)));
            }
            Ok((new_constraints, effective))
        }
        Operation::Clash(clash) => Err(TyError::Clash(ClashErr { lhe: clash.lhe, rhe: clash.rhe })),
        Operation::Occurs(occurs) => {
            Err(TyError::Occurs(OccursErr { lhe: occurs.lhe, rhe: occurs.rhe }))
        }
    }
}
