use crate::intern::Interner;
use biodivine_lib_bdd::Bdd;
use biodivine_lib_bdd::BddVariableSet;
use facts::LocalFacts;
use polonius_engine::FactTypes;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod facts;
mod intern;
mod parse;
mod tab_delim;

// TODO change order to speed up
static V: [&'static str; 8] = [
    "origin0", "origin1", "loan", "variable", "path0", "path1", "point0", "point1",
];

#[derive(Debug)]
pub struct Atom2Usize<T: FactTypes> {
    origin: HashMap<T::Origin, usize>,
    loan: HashMap<T::Loan, usize>,
    point: HashMap<T::Point, usize>,
    path: HashMap<T::Path, usize>,
    variable: HashMap<T::Variable, usize>,
}

fn main() {
    let facts_dir = "/home/lyj/polonius/inputs/vec-push-ref/nll-facts/main";
    let tables = &mut intern::InternerTables::new();
    let all_facts = tab_delim::load_tab_delimited_facts(tables, &Path::new(&facts_dir)).unwrap();

    let mut atom2usize = Atom2Usize::<LocalFacts> {
        origin: HashMap::new(),
        loan: HashMap::new(),
        point: HashMap::new(),
        path: HashMap::new(),
        variable: HashMap::new(),
    };
    for (o, l, p) in &all_facts.loan_issued_at {
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(o.clone(), atom2usize.origin.len());
        }
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(l.clone(), atom2usize.loan.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (p1, p2) in &all_facts.cfg_edge {
        if !atom2usize.point.contains_key(p1) {
            atom2usize.point.insert(p1.clone(), atom2usize.point.len());
        }
        if !atom2usize.point.contains_key(p2) {
            atom2usize.point.insert(p2.clone(), atom2usize.point.len());
        }
    }
    for (l, p) in &all_facts.loan_killed_at {
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(l.clone(), atom2usize.loan.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (o1, o2, p) in &all_facts.subset_base {
        if !atom2usize.origin.contains_key(o1) {
            atom2usize
                .origin
                .insert(o1.clone(), atom2usize.origin.len());
        }
        if !atom2usize.origin.contains_key(o2) {
            atom2usize
                .origin
                .insert(o2.clone(), atom2usize.origin.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (p, l) in &all_facts.loan_invalidated_at {
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(l.clone(), atom2usize.loan.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (v, p) in &all_facts.var_used_at {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(v.clone(), atom2usize.variable.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (v, p) in &all_facts.var_defined_at {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(v.clone(), atom2usize.variable.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (v, p) in &all_facts.var_dropped_at {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(v.clone(), atom2usize.variable.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(p.clone(), atom2usize.point.len());
        }
    }
    for (v, o) in &all_facts.use_of_var_derefs_origin {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(v.clone(), atom2usize.variable.len());
        }
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(o.clone(), atom2usize.origin.len());
        }
    }
    for (v, o) in &all_facts.drop_of_var_derefs_origin {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(v.clone(), atom2usize.variable.len());
        }
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(o.clone(), atom2usize.origin.len());
        }
    }
    for (p1, p2) in &all_facts.child_path {
        if !atom2usize.path.contains_key(p1) {
            atom2usize.path.insert(p1.clone(), atom2usize.path.len());
        }
        if !atom2usize.path.contains_key(p2) {
            atom2usize.path.insert(p2.clone(), atom2usize.path.len());
        }
    }
    for (p, v) in &all_facts.path_is_var {
        if !atom2usize.path.contains_key(p) {
            atom2usize.path.insert(p.clone(), atom2usize.path.len());
        }
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(v.clone(), atom2usize.variable.len());
        }
    }
    for (path, point) in &all_facts.path_assigned_at_base {
        if !atom2usize.path.contains_key(path) {
            atom2usize.path.insert(path.clone(), atom2usize.path.len());
        }
        if !atom2usize.point.contains_key(point) {
            atom2usize
                .point
                .insert(point.clone(), atom2usize.point.len());
        }
    }
    for (path, point) in &all_facts.path_moved_at_base {
        if !atom2usize.path.contains_key(path) {
            atom2usize.path.insert(path.clone(), atom2usize.path.len());
        }
        if !atom2usize.point.contains_key(point) {
            atom2usize
                .point
                .insert(point.clone(), atom2usize.point.len());
        }
    }
    for (path, point) in &all_facts.path_accessed_at_base {
        if !atom2usize.path.contains_key(path) {
            atom2usize.path.insert(path.clone(), atom2usize.path.len());
        }
        if !atom2usize.point.contains_key(point) {
            atom2usize
                .point
                .insert(point.clone(), atom2usize.point.len());
        }
    }
    for (o1, o2) in &all_facts.known_placeholder_subset {
        if !atom2usize.origin.contains_key(o1) {
            atom2usize
                .origin
                .insert(o1.clone(), atom2usize.origin.len());
        }
        if !atom2usize.origin.contains_key(o2) {
            atom2usize
                .origin
                .insert(o2.clone(), atom2usize.origin.len());
        }
    }
    for (o, l) in &all_facts.placeholder {
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(o.clone(), atom2usize.origin.len());
        }
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(l.clone(), atom2usize.loan.len());
        }
    }
    dump_map("origin", &atom2usize.origin, &tables.origins);
    dump_map("loan", &atom2usize.loan, &tables.loans);
    dump_map("path", &atom2usize.path, &tables.paths);
    dump_map("point", &atom2usize.point, &tables.points);
    dump_map("variable", &atom2usize.variable, &tables.variables);

    // origin1 origin2
    // point1 point2
    // path1 path2
    // loan variable
    let bddvar_count: usize = 2 * log2(atom2usize.origin.len().next_power_of_two())
        + log2(atom2usize.loan.len().next_power_of_two())
        + log2(atom2usize.variable.len().next_power_of_two())
        + 2 * log2(atom2usize.path.len().next_power_of_two())
        + 2 * log2(atom2usize.point.len().next_power_of_two());
    let variable_set = BddVariableSet::new_anonymous(bddvar_count as u16);

    let mut index: usize = 0;
    let mut mp: HashMap<&'static str, Vec<Bdd>> = HashMap::new();

    for s in V {
        match s {
            "origin0" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.origin.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "origin1" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.origin.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "loan" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.loan.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "variable" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.variable.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "path0" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.path.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "path1" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.path.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "point0" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.point.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            "point1" => {
                let mut v = vec![];
                for _i in 0..log2(atom2usize.point.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
                mp.insert(s, v);
            }
            _ => unreachable!(),
        }
    }

    dbg!("{:?}", &atom2usize);
    // dbg!("{:?}", &mp);

    let bdd: Bdd = parse::parse_cfg_edge(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "cfg_edge");

    let bdd: Bdd = parse::parse_child_path(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "child_path");

    let bdd: Bdd =
        parse::parse_drop_of_var_derefs_origin(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "drop_of_var_derefs_origin");

    let bdd: Bdd =
        parse::parse_use_of_var_derefs_origin(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "use_of_var_derefs_origin");

    let bdd: Bdd =
        parse::parse_known_placeholder_subset(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "known_placeholder_subset");

    let bdd: Bdd = parse::parse_loan_invalidated_at(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "loan_invalidated_at");

    let bdd: Bdd = parse::parse_loan_killed_at(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "loan_killed_at");

    let bdd: Bdd = parse::parse_loan_issued_at(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "loan_issued_at");

    let bdd: Bdd = parse::parse_path_accessed_at_base(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_accessed_at_base");

    let bdd: Bdd = parse::parse_path_assigned_at_base(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_assigned_at_base");

    let bdd: Bdd = parse::parse_path_moved_at_base(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_moved_at_base");

    let bdd: Bdd = parse::parse_path_is_var(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_is_var");

    let bdd: Bdd = parse::parse_placeholder(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "placeholder");

    let bdd: Bdd = parse::parse_subset_base(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "subset_base");

    let bdd: Bdd = parse::parse_var_defined_at(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "var_defined_at");

    let bdd: Bdd = parse::parse_var_dropped_at(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "var_dropped_at");

    let bdd: Bdd = parse::parse_var_used_at(&atom2usize, &all_facts, &mp, &variable_set);
    dump_bdd(bddvar_count, bdd, "var_used_at");
}

fn dump_bdd(bddvar_count: usize, bdd: Bdd, name: &str) -> std::io::Result<()> {
    std::fs::create_dir("pa.joeq");
    let mut path: String = "pa.joeq/".to_owned();
    path += name;
    path += ".bdd";
    let mut file = File::create(&path)?;
    // dbg!("{:?}{:?}",name,&bdd);
    if (bdd.size() == 1) {
        // always false
        file.write("0 0 0".as_bytes())?;
        return Ok(());
    }
    if (bdd.size() == 2) {
        // always true
        file.write("0 0 1".as_bytes())?;
        return Ok(());
    }
    // no 0 1
    file.write((bdd.size() - 2).to_string().as_bytes())?;
    file.write(" ".as_bytes())?;
    file.write(bddvar_count.to_string().as_bytes())?;
    file.write("\n".as_bytes())?;
    for i in 0..bddvar_count {
        file.write(i.to_string().as_bytes())?;
        file.write(" ".as_bytes())?;
    }
    file.write("\n".as_bytes())?;
    for (i, bddnode) in bdd.nodes().enumerate().skip(2) {
        file.write(i.to_string().as_bytes())?;
        file.write(" ".as_bytes())?;
        file.write(bddnode.var.0.to_string().as_bytes())?;
        file.write(" ".as_bytes())?;
        file.write(bddnode.low_link.0.to_string().as_bytes())?;
        file.write(" ".as_bytes())?;
        file.write(bddnode.high_link.0.to_string().as_bytes())?;
        file.write("\n".as_bytes())?;
    }
    Ok(())
}

// dump .map file
fn dump_map<T: From<usize> + Into<usize> + Copy>(
    name: &str,
    mp: &HashMap<T, usize>,
    interner: &Interner<T>,
) -> std::io::Result<()> {
    std::fs::create_dir("pa.joeq");
    let mut vec = vec![""; mp.len()];
    for (k, v) in mp {
        vec[v.clone()] = interner.untern(*k);
    }
    let mut path: String = "pa.joeq/".to_owned();
    path += name;
    path += ".map";
    let mut file = File::create(&path)?;
    for s in vec {
        file.write(s.as_bytes())?;
        file.write("\n".as_bytes())?;
    }
    Ok(())
}

fn log2(mut x: usize) -> usize {
    let mut res = 0;
    while ((x & 1) == 0) {
        res += 1;
        x >>= 1;
    }
    res
}

#[test]
fn test_log2() {
    assert_eq!(log2(1), 0);
    assert_eq!(log2(2), 1);
    assert_eq!(log2(4), 2);
}
