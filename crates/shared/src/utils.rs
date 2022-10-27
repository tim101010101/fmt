mod io;
mod path_resolve;

pub(crate) use {
    io::{read_file, write_file},
    path_resolve::resolve,
};
