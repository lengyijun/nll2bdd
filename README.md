In `nll2bdd`, `cargo r /path/to/nll-facts/main`

transform nll-facts to .bdd, .map, ....

output in `/pa.joeq`

```
svn checkout https://svn.code.sf.net/p/bddbddb/code/trunk bddbddb-code
```
In `bddbddb-code/bddbddb`,
```
/bin/java -mx512m -cp .:javabdd-2.0.jar:jwutil-1.0.jar:weka.jar:jdom.jar  net.sf.bddbddb.Interactive ~/nll2bdd/initialization.datalog
```

```
> solve
# initialization
> save ancestor_path,path_moved_at,path_assigned_at, path_accessed_at, path_begins_with_var, path_maybe_initialized_on_exit, path_maybe_uninitialized_on_exit, var_maybe_partly_initialized_on_exit, move_error

# liveness
> save ancestor_path,path_moved_at,path_assigned_at, path_accessed_at, path_begins_with_var, path_maybe_initialized_on_exit, path_maybe_uninitialized_on_exit, var_maybe_partly_initialized_on_exit, move_error, var_live_on_entry, var_maybe_partly_initialized_on_entry, var_drop_live_on_entry, origin_live_on_entry

# naive
> save ancestor_path,path_moved_at,path_assigned_at, path_accessed_at, path_begins_with_var, path_maybe_initialized_on_exit, path_maybe_uninitialized_on_exit, var_maybe_partly_initialized_on_exit, move_error, var_live_on_entry, var_maybe_partly_initialized_on_entry, var_drop_live_on_entry, origin_live_on_entry, subset, origin_contains_loan_on_entry, known_contains, subset_error, loan_live_at, errors
```

You can see `move_error.bdd` in `nll2bdd/pa.joeq/`


```
> move_error(X,Y)?
> move_error(60,Y)?
```
Read output in interactive mode.



|                                                                                   | datafrog | bddbddb |
| ----- | -----| -----|
| issue-29466.rs                                                     |    | 60s             |
| issue-29540.rs                                                     |                  | ?                 |
| issue-74564-if-expr-stack-overflow.rs       |            | >1h           |
| clap                                                                          |                   | 1h             |


`subset` take most of time

# TODO
parallel calculate bdd

