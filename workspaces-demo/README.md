### Workspaces
A `workspace` is a set of packages that share the same `Cargo.lock` and `output` directory.
To use `workspace` add the `workspace` section in the `Cargo.toml` file as shown in the 
example toml file in the module. `members` subsection list all the `crates` that share 
the corresponding `workspace`.  