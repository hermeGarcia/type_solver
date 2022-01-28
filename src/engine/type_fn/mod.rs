use crate::solver_interface::*;

pub fn is_eq(l: &Xtype, r: &Xtype) -> bool {
    match (l, r) {
        (Xtype::Var(ld), Xtype::Var(rd)) => ld.name == rd.name,
        (Xtype::Constant(ld), Xtype::Constant(rd)) => ld.name == rd.name,
        (Xtype::Function(ld), Xtype::Function(rd)) => {
            is_eq(&ld.to, &rd.to) && is_eq(&ld.from, &rd.from)
        }
        _ => false,
    }
}

pub fn is_contained(var: VarID, r: &Xtype) -> bool {
    match r {
        Xtype::Var(v) => v.name == var,
        Xtype::Function(f) => is_contained(var, &f.to) || is_contained(var, &f.from),
        _ => false,
    }
}
