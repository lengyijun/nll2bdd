use crate::{Atom2Usize, Var2Bdd};
use biodivine_lib_bdd::Bdd;
use biodivine_lib_bdd::BddVariableSet;
use polonius_engine::AllFacts;
use polonius_engine::FactTypes;

// cfg_edge(point0, point1)
pub fn parse_cfg_edge<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (p0, p1) in &all_facts.cfg_edge {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.point[p0];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[p1];
        for (n, b) in var2bdd.point1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// child_path(child, parent)
pub fn parse_child_path<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (p0, p1) in &all_facts.child_path {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.path[p0];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.path[p1];
        for (n, b) in var2bdd.path1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// drop_of_var_derefs_origin(var, origin)
pub fn parse_drop_of_var_derefs_origin<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (variable, o) in &all_facts.drop_of_var_derefs_origin {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// use_of_var_derefs_origin(variable, origin)
pub fn parse_use_of_var_derefs_origin<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (variable, o) in &all_facts.use_of_var_derefs_origin {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// known_placeholder_subset(origin1,origin2)
pub fn parse_known_placeholder_subset<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (o0, o1) in &all_facts.known_placeholder_subset {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.origin[o0];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.origin[o1];
        for (n, b) in var2bdd.origin1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// loan_invalidated_at: Vec<(T::Point, T::Loan)>,
pub fn parse_loan_invalidated_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (point, l) in &all_facts.loan_invalidated_at {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// loan_killed_at: Vec<(T::Loan, T::Point )>,
pub fn parse_loan_killed_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (l, point) in &all_facts.loan_killed_at {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

pub fn parse_loan_issued_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (o, l, point) in &all_facts.loan_issued_at {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// path_accessed_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_accessed_at_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (path, point) in &all_facts.path_accessed_at_base {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// path_assigned_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_assigned_at_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (path, point) in &all_facts.path_assigned_at_base {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// path_moved_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_moved_at_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (path, point) in &all_facts.path_moved_at_base {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// path_is_var: Vec<(T::Path, T::Variable)>
pub fn parse_path_is_var<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (path, variable) in &all_facts.path_is_var {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// placeholder: Vec<(T::Origin, T::Loan)>,
pub fn parse_placeholder<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (o, l) in &all_facts.placeholder {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// subset_base: Vec<(T::Origin, T::Origin, T::Point)>
pub fn parse_subset_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (o0, o1, point) in &all_facts.subset_base {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.origin[o0];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.origin[o1];
        for (n, b) in var2bdd.origin1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

// var_defined_at: Vec<(T::Variable, T::Point)>
pub fn parse_var_defined_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (variable, point) in &all_facts.var_defined_at {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

pub fn parse_var_dropped_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (variable, point) in &all_facts.var_dropped_at {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}

pub fn parse_var_used_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let mut res = bdd_variable_set.mk_false();
    for (variable, point) in &all_facts.var_used_at {
        let mut temp = bdd_variable_set.mk_true();
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                temp = temp.and(b);
            } else {
                temp = temp.and(&b.not());
            }
        }
        res = res.or(&temp);
    }
    res
}
