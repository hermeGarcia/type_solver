use crate::solver_interface::*;
use crate::engine::instructions::*;
use crate::engine::type_fn;

fn rule_var(var: VarData, t: Xtype) -> OperationList {
    let mut operations = OperationList::new();
    if type_fn::is_contained(var.name, &t) {
        operations.push_back(Operation::Occurs(OccursOp { rhe: Xtype::Var(var), lhe: t }));
    } else {
        operations.push_back(Operation::Replace(ReplaceOp {
            lhe: Xtype::Var(var.clone()),
            rhe: t.clone(),
        }));
        operations.push_back(Operation::Add(AddOp { is_new: false, elem: Constraint::from((Xtype::Var(var), t)) }));
    }
    operations
}

fn rule_decompose(lf: FunctionData, rf: FunctionData) -> OperationList {
    let mut operations = OperationList::new();
    operations.push_back(Operation::Add(AddOp { is_new: true, elem: Constraint::from((*lf.to, *rf.to)) }));
    operations.push_back(Operation::Add(AddOp { is_new: true, elem: Constraint::from((*lf.from, *rf.from)) }));
    operations
}

fn rule_sym(t: Xtype, var: VarData) -> OperationList {
    let mut operations = OperationList::new();
    operations.push_back(Operation::Add(AddOp { is_new: true, elem: Constraint::from((Xtype::Var(var), t)) }));
    operations
}

fn rule_clash(l: Xtype, r: Xtype) -> OperationList {
    let mut operations = OperationList::new();
    operations.push_back(Operation::Clash(ClashOp { lhe: l, rhe: r }));
    operations
}

pub fn constraint_to_operations(constraint: Constraint) -> OperationList {
    match constraint.into() {
        (Xtype::Var(var), t) => rule_var(var, t),
        (t, Xtype::Var(var)) => rule_sym(t, var),
        (Xtype::Function(lf), Xtype::Function(rf)) => rule_decompose(lf, rf),
        (t1, t2) if type_fn::is_eq(&t1, &t2) => OperationList::new(),
        (t1, t2) => rule_clash(t1, t2),
    }
}
