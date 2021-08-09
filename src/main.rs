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
static BDDVARORDER: [&'static str; 8] = [
    "ORIGIN0", "ORIGIN1", "LOAN", "VARIABLE", "PATH0", "PATH1", "POINT0", "POINT1",
];

#[derive(Debug)]
pub struct Atom2Usize<T: FactTypes> {
    origin: HashMap<T::Origin, usize>,
    loan: HashMap<T::Loan, usize>,
    point: HashMap<T::Point, usize>,
    path: HashMap<T::Path, usize>,
    variable: HashMap<T::Variable, usize>,
}

pub struct Var2Bdd {
    origin0: Vec<Bdd>,
    origin1: Vec<Bdd>,
    path0: Vec<Bdd>,
    path1: Vec<Bdd>,
    point0: Vec<Bdd>,
    point1: Vec<Bdd>,
    variable: Vec<Bdd>,
    loan: Vec<Bdd>,
}

fn main() {
    // let facts_dir = "/home/lyj/polonius/inputs/smoke-test/nll-facts/basic_move_error";
    let facts_dir = std::env::args().nth(1).unwrap();
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
            atom2usize.origin.insert(*o, atom2usize.origin.len());
        }
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(*l, atom2usize.loan.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (p1, p2) in &all_facts.cfg_edge {
        if !atom2usize.point.contains_key(p1) {
            atom2usize.point.insert(*p1, atom2usize.point.len());
        }
        if !atom2usize.point.contains_key(p2) {
            atom2usize.point.insert(*p2, atom2usize.point.len());
        }
    }
    for (l, p) in &all_facts.loan_killed_at {
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(*l, atom2usize.loan.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (o1, o2, p) in &all_facts.subset_base {
        if !atom2usize.origin.contains_key(o1) {
            atom2usize
                .origin
                .insert(*o1, atom2usize.origin.len());
        }
        if !atom2usize.origin.contains_key(o2) {
            atom2usize
                .origin
                .insert(*o2, atom2usize.origin.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (p, l) in &all_facts.loan_invalidated_at {
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(*l, atom2usize.loan.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (v, p) in &all_facts.var_used_at {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(*v, atom2usize.variable.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (v, p) in &all_facts.var_defined_at {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(*v, atom2usize.variable.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (v, p) in &all_facts.var_dropped_at {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(*v, atom2usize.variable.len());
        }
        if !atom2usize.point.contains_key(p) {
            atom2usize.point.insert(*p, atom2usize.point.len());
        }
    }
    for (v, o) in &all_facts.use_of_var_derefs_origin {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(*v, atom2usize.variable.len());
        }
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(*o, atom2usize.origin.len());
        }
    }
    for (v, o) in &all_facts.drop_of_var_derefs_origin {
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(*v, atom2usize.variable.len());
        }
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(*o, atom2usize.origin.len());
        }
    }
    for (p1, p2) in &all_facts.child_path {
        if !atom2usize.path.contains_key(p1) {
            atom2usize.path.insert(*p1, atom2usize.path.len());
        }
        if !atom2usize.path.contains_key(p2) {
            atom2usize.path.insert(*p2, atom2usize.path.len());
        }
    }
    for (p, v) in &all_facts.path_is_var {
        if !atom2usize.path.contains_key(p) {
            atom2usize.path.insert(*p, atom2usize.path.len());
        }
        if !atom2usize.variable.contains_key(v) {
            atom2usize
                .variable
                .insert(*v, atom2usize.variable.len());
        }
    }
    for (path, point) in &all_facts.path_assigned_at_base {
        if !atom2usize.path.contains_key(path) {
            atom2usize.path.insert(*path, atom2usize.path.len());
        }
        if !atom2usize.point.contains_key(point) {
            atom2usize
                .point
                .insert(*point, atom2usize.point.len());
        }
    }
    for (path, point) in &all_facts.path_moved_at_base {
        if !atom2usize.path.contains_key(path) {
            atom2usize.path.insert(*path, atom2usize.path.len());
        }
        if !atom2usize.point.contains_key(point) {
            atom2usize
                .point
                .insert(*point, atom2usize.point.len());
        }
    }
    for (path, point) in &all_facts.path_accessed_at_base {
        if !atom2usize.path.contains_key(path) {
            atom2usize.path.insert(*path, atom2usize.path.len());
        }
        if !atom2usize.point.contains_key(point) {
            atom2usize
                .point
                .insert(*point, atom2usize.point.len());
        }
    }
    for (o1, o2) in &all_facts.known_placeholder_subset {
        if !atom2usize.origin.contains_key(o1) {
            atom2usize
                .origin
                .insert(*o1, atom2usize.origin.len());
        }
        if !atom2usize.origin.contains_key(o2) {
            atom2usize
                .origin
                .insert(*o2, atom2usize.origin.len());
        }
    }
    for (o, l) in &all_facts.placeholder {
        if !atom2usize.origin.contains_key(o) {
            atom2usize.origin.insert(*o, atom2usize.origin.len());
        }
        if !atom2usize.loan.contains_key(l) {
            atom2usize.loan.insert(*l, atom2usize.loan.len());
        }
    }

    dump_map("origin", &atom2usize.origin, &tables.origins);
    dump_map("loan", &atom2usize.loan, &tables.loans);
    dump_map("path", &atom2usize.path, &tables.paths);
    dump_map("point", &atom2usize.point, &tables.points);
    dump_map("variable", &atom2usize.variable, &tables.variables);
    dump_fielddomainspa(&atom2usize);

    // origin0 origin1
    // point0 point1
    // path0 path1
    // loan variable
    let bddvar_count: usize = 2 * log2(atom2usize.origin.len().next_power_of_two())
        + log2(atom2usize.loan.len().next_power_of_two())
        + log2(atom2usize.variable.len().next_power_of_two())
        + 2 * log2(atom2usize.path.len().next_power_of_two())
        + 2 * log2(atom2usize.point.len().next_power_of_two());
    let variable_set = BddVariableSet::new_anonymous(bddvar_count as u16);

    let mut index: usize = 0;
    let mut var2bdd = Var2Bdd {
        origin0: vec![],
        origin1: vec![],
        path0: vec![],
        path1: vec![],
        point0: vec![],
        point1: vec![],
        variable: vec![],
        loan: vec![],
    };

    for s in BDDVARORDER {
        match s {
            "ORIGIN0" => {
                let v = &mut var2bdd.origin0;
                for _i in 0..log2(atom2usize.origin.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "ORIGIN1" => {
                let v = &mut var2bdd.origin1;
                for _i in 0..log2(atom2usize.origin.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "LOAN" => {
                let v = &mut var2bdd.loan;
                for _i in 0..log2(atom2usize.loan.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "VARIABLE" => {
                let v = &mut var2bdd.variable;
                for _i in 0..log2(atom2usize.variable.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "PATH0" => {
                let v = &mut var2bdd.path0;
                for _i in 0..log2(atom2usize.path.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "PATH1" => {
                let v = &mut var2bdd.path1;
                for _i in 0..log2(atom2usize.path.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "POINT0" => {
                let v = &mut var2bdd.point0;
                for _i in 0..log2(atom2usize.point.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            "POINT1" => {
                let v = &mut var2bdd.point1;
                for _i in 0..log2(atom2usize.point.len().next_power_of_two()) {
                    v.push(variable_set.mk_var_by_name(&("x_".to_owned() + &index.to_string())));
                    index += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    // dbg!("{:?}", &atom2usize);
    // dbg!("{:?}", &var2bdd);

    let bdd: Bdd = parse::parse_cfg_edge(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "cfg_edge");

    let bdd: Bdd = parse::parse_child_path(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "child_path");

    let bdd: Bdd =
        parse::parse_drop_of_var_derefs_origin(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "drop_of_var_derefs_origin");

    let bdd: Bdd =
        parse::parse_use_of_var_derefs_origin(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "use_of_var_derefs_origin");

    let bdd: Bdd =
        parse::parse_known_placeholder_subset(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "known_placeholder_subset");

    let bdd: Bdd =
        parse::parse_loan_invalidated_at(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "loan_invalidated_at");

    let bdd: Bdd = parse::parse_loan_killed_at(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "loan_killed_at");

    let bdd: Bdd = parse::parse_loan_issued_at(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "loan_issued_at");

    let bdd: Bdd =
        parse::parse_path_accessed_at_base(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_accessed_at_base");

    let bdd: Bdd =
        parse::parse_path_assigned_at_base(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_assigned_at_base");

    let bdd: Bdd =
        parse::parse_path_moved_at_base(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_moved_at_base");

    let bdd: Bdd = parse::parse_path_is_var(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "path_is_var");

    let bdd: Bdd = parse::parse_placeholder(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "placeholder");

    let bdd: Bdd = parse::parse_subset_base(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "subset_base");

    let bdd: Bdd = parse::parse_var_defined_at(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "var_defined_at");

    let bdd: Bdd = parse::parse_var_dropped_at(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "var_dropped_at");

    let bdd: Bdd = parse::parse_var_used_at(&atom2usize, &all_facts, &var2bdd, &variable_set);
    dump_bdd(bddvar_count, bdd, "var_used_at");
}

fn dump_bdd(bddvar_count: usize, bdd: Bdd, name: &str) -> std::io::Result<()> {
    std::fs::create_dir("pa.joeq");
    let mut path: String = "pa.joeq/".to_owned();
    path += name;
    path += ".bdd";
    let mut file = File::create(&path)?;
    if bdd.size() == 1 {
        // always false
        file.write_all("0 0 0".as_bytes())?;
        return Ok(());
    }
    if bdd.size() == 2 {
        // always true
        file.write_all("0 0 1".as_bytes())?;
        return Ok(());
    }
    // no 0 1
    file.write_all((bdd.size() - 2).to_string().as_bytes())?;
    file.write_all(" ".as_bytes())?;
    file.write_all(bddvar_count.to_string().as_bytes())?;
    file.write_all("\n".as_bytes())?;
    for i in 0..bddvar_count {
        file.write_all(i.to_string().as_bytes())?;
        file.write_all(" ".as_bytes())?;
    }
    file.write_all("\n".as_bytes())?;
    for (i, bddnode) in bdd.nodes().enumerate().skip(2) {
        file.write_all(i.to_string().as_bytes())?;
        file.write_all(" ".as_bytes())?;
        file.write_all(bddnode.var.0.to_string().as_bytes())?;
        file.write_all(" ".as_bytes())?;
        file.write_all(bddnode.low_link.0.to_string().as_bytes())?;
        file.write_all(" ".as_bytes())?;
        file.write_all(bddnode.high_link.0.to_string().as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }
    Ok(())
}

// dump .map file
fn dump_map<T: From<usize> + Into<usize> + Copy>(
    name: &str,
    var2bdd: &HashMap<T, usize>,
    interner: &Interner<T>,
) -> std::io::Result<()> {
    std::fs::create_dir("pa.joeq");
    let mut path: String = "pa.joeq/".to_owned();
    path += name;
    path += ".map";
    let mut file = File::create(&path)?;

    let mut vec = vec![""; var2bdd.len()];
    for (k, v) in var2bdd {
        vec[v.clone()] = interner.untern(*k);
    }
    for s in vec {
        file.write_all(s.as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }
    Ok(())
}

fn dump_fielddomainspa<T: FactTypes>(atom2usize: &Atom2Usize<T>) -> std::io::Result<()> {
    std::fs::create_dir("pa.joeq");
    let path = "pa.joeq/fielddomains.pa";
    let mut file = File::create(path)?;

    file.write_all("VARIABLE ".as_bytes())?;
    file.write_all(
        atom2usize
            .variable
            .len()
            .next_power_of_two()
            .to_string()
            .as_bytes(),
    )?;
    file.write_all(" variable.map\n".as_bytes())?;

    file.write_all("ORIGIN ".as_bytes())?;
    file.write_all(
        atom2usize
            .origin
            .len()
            .next_power_of_two()
            .to_string()
            .as_bytes(),
    )?;
    file.write_all(" origin.map\n".as_bytes())?;

    file.write_all("POINT ".as_bytes())?;
    file.write_all(
        atom2usize
            .point
            .len()
            .next_power_of_two()
            .to_string()
            .as_bytes(),
    )?;
    file.write_all(" point.map\n".as_bytes())?;

    file.write_all("PATH ".as_bytes())?;
    file.write_all(
        atom2usize
            .path
            .len()
            .next_power_of_two()
            .to_string()
            .as_bytes(),
    )?;
    file.write_all(" path.map\n".as_bytes())?;

    file.write_all("LOAN ".as_bytes())?;
    file.write_all(
        atom2usize
            .loan
            .len()
            .next_power_of_two()
            .to_string()
            .as_bytes(),
    )?;
    file.write_all(" loan.map\n".as_bytes())?;

    Ok(())
}

fn log2(mut x: usize) -> usize {
    let mut res = 0;
    while (x & 1) == 0 {
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
