use crate::{Atom2Usize, Var2Bdd};
use biodivine_lib_bdd::Bdd;
use biodivine_lib_bdd::BddVariable;
use biodivine_lib_bdd::BddVariableSet;
use polonius_engine::AllFacts;
use polonius_engine::FactTypes;

fn prepare(inputs: Vec<&Vec<u16>>) -> (Vec<BddVariable>, [usize; 128]) {
    let mut v: Vec<u16> = vec![];
    for input in inputs {
        v.extend(input);
    }
    v.sort_unstable();
    let variables: Vec<BddVariable> = v.iter().map(|i| BddVariable(*i)).collect();
    // at most 1<<16
    let mut mp = [0usize; 128];
    for (i, y) in v.into_iter().enumerate() {
        mp[y as usize] = i;
    }
    (variables, mp)
}

// cfg_edge(point0, point1)
pub fn parse_cfg_edge<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.point0, &var2bdd.point1]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.cfg_edge.len());

    for (p0, p1) in &all_facts.cfg_edge {
        let mut v = vec![false; length];

        let x: usize = atom2usize.point[p0];
        for (i, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << i)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }

        let x: usize = atom2usize.point[p1];
        for (i, b) in var2bdd.point1.iter().enumerate() {
            if (x & (1 << i)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }

        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// child_path(child, parent)
pub fn parse_child_path<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.path0, &var2bdd.path1]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.child_path.len());

    for (p0, p1) in &all_facts.child_path {
        let mut v = vec![false; length];

        let x: usize = atom2usize.path[p0];
        for (i, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << i)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.path[p1];
        for (n, b) in var2bdd.path1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// drop_of_var_derefs_origin(var, origin)
pub fn parse_drop_of_var_derefs_origin<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.variable, &var2bdd.origin0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.drop_of_var_derefs_origin.len());

    for (variable, o) in &all_facts.drop_of_var_derefs_origin {
        let mut v = vec![false; length];

        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// use_of_var_derefs_origin(variable, origin)
pub fn parse_use_of_var_derefs_origin<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.variable, &var2bdd.origin0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.use_of_var_derefs_origin.len());

    for (variable, o) in &all_facts.use_of_var_derefs_origin {
        let mut v = vec![false; length];

        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// known_placeholder_subset(origin1,origin2)
pub fn parse_known_placeholder_subset<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.origin0, &var2bdd.origin1]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.known_placeholder_subset.len());

    for (o0, o1) in &all_facts.known_placeholder_subset {
        let mut v = vec![false; length];

        let x: usize = atom2usize.origin[o0];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.origin[o1];
        for (n, b) in var2bdd.origin1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// loan_invalidated_at: Vec<(T::Point, T::Loan)>,
pub fn parse_loan_invalidated_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.point0, &var2bdd.loan]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.loan_invalidated_at.len());

    for (point, l) in &all_facts.loan_invalidated_at {
        let mut v = vec![false; length];

        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// loan_killed_at: Vec<(T::Loan, T::Point )>,
pub fn parse_loan_killed_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.point0, &var2bdd.loan]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.loan_killed_at.len());

    for (l, point) in &all_facts.loan_killed_at {
        let mut v = vec![false; length];

        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

pub fn parse_loan_issued_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.origin0, &var2bdd.loan, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.loan_issued_at.len());

    for (o, l, point) in &all_facts.loan_issued_at {
        let mut v = vec![false; length];

        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// path_accessed_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_accessed_at_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.path0, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.path_accessed_at_base.len());

    for (path, point) in &all_facts.path_accessed_at_base {
        let mut v = vec![false; length];

        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// path_assigned_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_assigned_at_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.path0, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.path_assigned_at_base.len());

    for (path, point) in &all_facts.path_assigned_at_base {
        let mut v = vec![false; length];

        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// path_moved_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_moved_at_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.path0, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.path_moved_at_base.len());

    for (path, point) in &all_facts.path_moved_at_base {
        let mut v = vec![false; length];

        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// path_is_var: Vec<(T::Path, T::Variable)>
pub fn parse_path_is_var<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.path0, &var2bdd.variable]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.path_is_var.len());

    for (path, variable) in &all_facts.path_is_var {
        let mut v = vec![false; length];

        let x: usize = atom2usize.path[path];
        for (n, b) in var2bdd.path0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// placeholder: Vec<(T::Origin, T::Loan)>,
pub fn parse_placeholder<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.origin0, &var2bdd.loan]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.placeholder.len());

    for (o, l) in &all_facts.placeholder {
        let mut v = vec![false; length];

        let x: usize = atom2usize.origin[o];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.loan[l];
        for (n, b) in var2bdd.loan.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// subset_base: Vec<(T::Origin, T::Origin, T::Point)>
pub fn parse_subset_base<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.origin0, &var2bdd.origin1, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.subset_base.len());

    for (o0, o1, point) in &all_facts.subset_base {
        let mut v = vec![false; length];

        let x: usize = atom2usize.origin[o0];
        for (n, b) in var2bdd.origin0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.origin[o1];
        for (n, b) in var2bdd.origin1.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

// var_defined_at: Vec<(T::Variable, T::Point)>
pub fn parse_var_defined_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.variable, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.var_defined_at.len());

    for (variable, point) in &all_facts.var_defined_at {
        let mut v = vec![false; length];

        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

pub fn parse_var_dropped_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.variable, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.var_dropped_at.len());

    for (variable, point) in &all_facts.var_dropped_at {
        let mut v = vec![false; length];

        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}

pub fn parse_var_used_at<T: FactTypes>(
    atom2usize: &Atom2Usize<T>,
    all_facts: &AllFacts<T>,
    var2bdd: &Var2Bdd,
    bdd_variable_set: &BddVariableSet,
) -> Bdd {
    let (variables, mp) = prepare(vec![&var2bdd.variable, &var2bdd.point0]);
    let length = variables.len();
    let mut vv = Vec::with_capacity(all_facts.var_used_at.len());

    for (variable, point) in &all_facts.var_used_at {
        let mut v = vec![false; length];

        let x: usize = atom2usize.variable[variable];
        for (n, b) in var2bdd.variable.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        let x: usize = atom2usize.point[point];
        for (n, b) in var2bdd.point0.iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v[mp[*b as usize]] = true;
            } else {
                v[mp[*b as usize]] = false;
            }
        }
        vv.push(v);
    }
    Bdd::dnf(bdd_variable_set, variables, vv)
}
