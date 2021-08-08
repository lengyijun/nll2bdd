use crate::Count;
use biodivine_lib_bdd::Bdd;
use biodivine_lib_bdd::BddVariableSet;
use polonius_engine::AllFacts;
use polonius_engine::FactTypes;
use std::collections::HashMap;

// cfg_edge(point1, point2)
pub fn parse_cfg_edge<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (p1, p2) in &all_facts.cfg_edge {
        v.clear();
        let x: usize = count.point[p1];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[p2];
        for (n, b) in mp["point2"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// child_path(child, parent)
pub fn parse_child_path<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (p1, p2) in &all_facts.child_path {
        v.clear();
        let x: usize = count.path[p1];
        for (n, b) in mp["path1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.path[p2];
        for (n, b) in mp["path2"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// drop_of_var_derefs_origin(var, origin)
pub fn parse_drop_of_var_derefs_origin<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (variable, o) in &all_facts.drop_of_var_derefs_origin {
        v.clear();
        let x: usize = count.variable[variable];
        for (n, b) in mp["var"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.origin[o];
        for (n, b) in mp["origin1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// use_of_var_derefs_origin(variable, origin)
pub fn parse_use_of_var_derefs_origin<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (variable, o) in &all_facts.use_of_var_derefs_origin {
        v.clear();
        let x: usize = count.variable[variable];
        for (n, b) in mp["var"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.origin[o];
        for (n, b) in mp["origin1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// known_placeholder_subset(origin1,origin2)
pub fn parse_known_placeholder_subset<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (o1, o2) in &all_facts.known_placeholder_subset {
        v.clear();
        let x: usize = count.origin[o1];
        for (n, b) in mp["origin1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.origin[o2];
        for (n, b) in mp["origin2"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// loan_invalidated_at: Vec<(T::Point, T::Loan)>,
pub fn parse_loan_invalidated_at<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (point, l) in &all_facts.loan_invalidated_at {
        v.clear();
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.loan[l];
        for (n, b) in mp["loan"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// loan_killed_at: Vec<(T::Loan, T::Point )>,
pub fn parse_loan_killed_at<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (l, point) in &all_facts.loan_killed_at {
        v.clear();
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.loan[l];
        for (n, b) in mp["loan"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

pub fn parse_loan_issued_at<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (o, l, point) in &all_facts.loan_issued_at {
        v.clear();
        let x: usize = count.origin[o];
        for (n, b) in mp["origin1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.loan[l];
        for (n, b) in mp["loan"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// path_accessed_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_accessed_at_base<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (path, point) in &all_facts.path_accessed_at_base {
        v.clear();
        let x: usize = count.path[path];
        for (n, b) in mp["path1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// path_assigned_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_assigned_at_base<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (path, point) in &all_facts.path_assigned_at_base {
        v.clear();
        let x: usize = count.path[path];
        for (n, b) in mp["path1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// path_moved_at_base: Vec<(T::Path, T::Point)>
pub fn parse_path_moved_at_base<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (path, point) in &all_facts.path_moved_at_base {
        v.clear();
        let x: usize = count.path[path];
        for (n, b) in mp["path1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// path_is_var: Vec<(T::Path, T::Variable)>
pub fn parse_path_is_var<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (path, variable) in &all_facts.path_is_var {
        v.clear();
        let x: usize = count.path[path];
        for (n, b) in mp["path1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.variable[variable];
        for (n, b) in mp["variable"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// placeholder: Vec<(T::Origin, T::Loan)>,
pub fn parse_placeholder<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (o, l) in &all_facts.placeholder {
        v.clear();
        let x: usize = count.origin[o];
        for (n, b) in mp["origin1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.loan[l];
        for (n, b) in mp["loan"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// subset_base: Vec<(T::Origin, T::Origin, T::Point)>
pub fn parse_subset_base<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (o1, o2, point) in &all_facts.subset_base {
        v.clear();
        let x: usize = count.origin[o1];
        for (n, b) in mp["origin1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.origin[o2];
        for (n, b) in mp["origin2"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

// var_defined_at: Vec<(T::Variable, T::Point)>
pub fn parse_var_defined_at<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (variable, point) in &all_facts.var_defined_at {
        v.clear();
        let x: usize = count.variable[variable];
        for (n, b) in mp["variable"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

pub fn parse_var_dropped_at<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (variable, point) in &all_facts.var_dropped_at {
        v.clear();
        let x: usize = count.variable[variable];
        for (n, b) in mp["variable"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}

pub fn parse_var_used_at<T: FactTypes>(
    count: &Count<T>,
    all_facts: &AllFacts<T>,
    mp: &HashMap<&'static str, Vec<Bdd>>,
    variables: &BddVariableSet,
) -> Bdd {
    let mut vv: Vec<Bdd> = vec![];
    let mut v: Vec<Bdd> = vec![];
    for (variable, point) in &all_facts.var_used_at {
        v.clear();
        let x: usize = count.variable[variable];
        for (n, b) in mp["variable"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let x: usize = count.point[point];
        for (n, b) in mp["point1"].iter().enumerate() {
            if (x & (1 << n)) > 0 {
                v.push(b.clone());
            } else {
                v.push(b.not());
            }
        }
        let b = v.iter().fold(variables.mk_true(), |a, b| a.and(b));
        vv.push(b);
    }
    vv.iter().fold(variables.mk_false(), |a, b| a.or(b))
}
