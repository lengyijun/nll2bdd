use facts::LocalFacts;
use once_cell::sync::OnceCell;
use polonius_engine::FactTypes;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod facts;
mod intern;
mod tab_delim;

// static INSTANCE: OnceCell<HashMap<usize, usize>> = OnceCell::new();

// point1,point2,
// origin1, origin2,
// loan
// var
// path1,path2
static V: [&'static str; 8] = [
    "origin1", "origin2", "loan", "var", "path1", "path2", "point1", "point2",
];

// count_bit:HashMap<&'static,usize>
// mp:HashMap<&'static str,Vec<usize>>::new()
// next_power_of_two()

#[derive(Debug)]
struct Count<T: FactTypes> {
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

    let mut count = Count::<LocalFacts> {
        origin: HashMap::new(),
        loan: HashMap::new(),
        point: HashMap::new(),
        path: HashMap::new(),
        variable: HashMap::new(),
    };
    for (o, l, p) in &all_facts.loan_issued_at {
        if !count.origin.contains_key(o) {
            count.origin.insert(o.clone(), count.origin.len());
        }
        if !count.loan.contains_key(l) {
            count.loan.insert(l.clone(), count.loan.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (p1, p2) in &all_facts.cfg_edge {
        if !count.point.contains_key(p1) {
            count.point.insert(p1.clone(), count.point.len());
        }
        if !count.point.contains_key(p2) {
            count.point.insert(p2.clone(), count.point.len());
        }
    }
    for (l, p) in &all_facts.loan_killed_at {
        if !count.loan.contains_key(l) {
            count.loan.insert(l.clone(), count.loan.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (o1, o2, p) in &all_facts.subset_base {
        if !count.origin.contains_key(o1) {
            count.origin.insert(o1.clone(), count.origin.len());
        }
        if !count.origin.contains_key(o2) {
            count.origin.insert(o2.clone(), count.origin.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (p, l) in &all_facts.loan_invalidated_at {
        if !count.loan.contains_key(l) {
            count.loan.insert(l.clone(), count.loan.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (v, p) in &all_facts.var_used_at {
        if !count.variable.contains_key(v) {
            count.variable.insert(v.clone(), count.variable.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (v, p) in &all_facts.var_defined_at {
        if !count.variable.contains_key(v) {
            count.variable.insert(v.clone(), count.variable.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (v, p) in &all_facts.var_dropped_at {
        if !count.variable.contains_key(v) {
            count.variable.insert(v.clone(), count.variable.len());
        }
        if !count.point.contains_key(p) {
            count.point.insert(p.clone(), count.point.len());
        }
    }
    for (v, o) in &all_facts.use_of_var_derefs_origin {
        if !count.variable.contains_key(v) {
            count.variable.insert(v.clone(), count.variable.len());
        }
        if !count.origin.contains_key(o) {
            count.origin.insert(o.clone(), count.origin.len());
        }
    }
    for (v, o) in &all_facts.drop_of_var_derefs_origin {
        if !count.variable.contains_key(v) {
            count.variable.insert(v.clone(), count.variable.len());
        }
        if !count.origin.contains_key(o) {
            count.origin.insert(o.clone(), count.origin.len());
        }
    }
    for (p1, p2) in &all_facts.child_path {
        if !count.path.contains_key(p1) {
            count.path.insert(p1.clone(), count.path.len());
        }
        if !count.path.contains_key(p2) {
            count.path.insert(p2.clone(), count.path.len());
        }
    }
    for (p, v) in &all_facts.path_is_var {
        if !count.path.contains_key(p) {
            count.path.insert(p.clone(), count.path.len());
        }
        if !count.variable.contains_key(v) {
            count.variable.insert(v.clone(), count.variable.len());
        }
    }
    for (path, point) in &all_facts.path_assigned_at_base {
        if !count.path.contains_key(path) {
            count.path.insert(path.clone(), count.path.len());
        }
        if !count.point.contains_key(point) {
            count.point.insert(point.clone(), count.point.len());
        }
    }
    for (path, point) in &all_facts.path_moved_at_base {
        if !count.path.contains_key(path) {
            count.path.insert(path.clone(), count.path.len());
        }
        if !count.point.contains_key(point) {
            count.point.insert(point.clone(), count.point.len());
        }
    }
    for (path, point) in &all_facts.path_accessed_at_base {
        if !count.path.contains_key(path) {
            count.path.insert(path.clone(), count.path.len());
        }
        if !count.point.contains_key(point) {
            count.point.insert(point.clone(), count.point.len());
        }
    }
    for (o1, o2) in &all_facts.known_placeholder_subset {
        if !count.origin.contains_key(o1) {
            count.origin.insert(o1.clone(), count.origin.len());
        }
        if !count.origin.contains_key(o2) {
            count.origin.insert(o2.clone(), count.origin.len());
        }
    }
    for (o, l) in &all_facts.placeholder {
        if !count.origin.contains_key(o) {
            count.origin.insert(o.clone(), count.origin.len());
        }
        if !count.loan.contains_key(l) {
            count.loan.insert(l.clone(), count.loan.len());
        }
    }
    dump_map("origin", &count.origin);
    dump_map("loan", &count.loan);
    dump_map("path", &count.path);
    dump_map("point", &count.point);
    dump_map("variable", &count.variable);

    let mut mp: HashMap<&'static str, Vec<usize>> = HashMap::new();
    // distinguish with 0/1 in BDD
    let mut index: usize = 2;

    for s in V {
        match s {
            "origin1" => {
                let mut v = vec![];
                for _i in 0..log2(count.origin.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "origin2" => {
                let mut v = vec![];
                for _i in 0..log2(count.origin.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "loan" => {
                let mut v = vec![];
                for _i in 0..log2(count.loan.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "var" => {
                let mut v = vec![];
                for _i in 0..log2(count.variable.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "path1" => {
                let mut v = vec![];
                for _i in 0..log2(count.path.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "path2" => {
                let mut v = vec![];
                for _i in 0..log2(count.path.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "point1" => {
                let mut v = vec![];
                for _i in 0..log2(count.point.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            "point2" => {
                let mut v = vec![];
                for _i in 0..log2(count.point.len().next_power_of_two()) {
                    v.push(index);
                    index += 1;
                }
                mp.insert(s, v);
            }
            _ => unreachable!(),
        }
    }

    dbg!("{:?}", mp);

    // INSTANCE.get_or_init(|| {
    // let mut m = HashMap::new();
    // m.insert(13, 12);
    // m
    // });
}

fn log2(mut x: usize) -> usize {
    let mut res = 0;
    while ((x & 1) == 0) {
        res += 1;
        x >>= 1;
    }
    res
}

// dump .map file
fn dump_map<T: Debug>(name: &str, mp: &HashMap<T, usize>) -> std::io::Result<()> {
    std::fs::create_dir("pa.joeq");
    let mut vec = vec![String::new(); mp.len()];
    for (k, v) in mp {
        vec[v.clone()] = format!("{:?}", k);
    }
    let mut path:String="pa.joeq/".to_owned();
    path+=name;
    path+=".map";
    let mut file = File::create(&path)?;
    for s in vec {
        file.write(s.as_bytes())?;
        file.write("\n".as_bytes())?;
    }
    Ok(())
}

#[test]
fn test_log2() {
    assert_eq!(log2(1), 0);
    assert_eq!(log2(2), 1);
    assert_eq!(log2(4), 2);
}
