use std::fs;
use heed::{EnvOpenOptions, Database};
use heed::types::*;

pub mod db_conf {
    pub const CREATE_DIR: &str = "target/heed.mdb";
    pub const MDB_FILE: &str = "target/heed.mdb";
}

pub struct Index<T: 'static> {
    pub env: heed::Env,
    pub main: Database<Str, SerdeJson<T>>,
}

impl<T> Index<T> {
    pub fn new() -> Result<Self, heed::Error> {
        use db_conf::*;

        fs::create_dir_all(CREATE_DIR)?;
        let env = EnvOpenOptions::new().open(MDB_FILE)?;

        // create_database will find or create one. So no need for 'open' function
        // (heed crate) create_database -> (lmdb-sys) crate mdb_dbi_open() -> C code
        // > A database handle denotes the name and parameters of a database,
        // > independently of whether such a database exists.
        // > The database handle may be discarded by calling mdb_dbi_close().
        // > The old database handle is returned if the database was already open.
        // > The handle may only be closed once.
        // cf) http://www.lmdb.tech/doc/group__internal.html#gac08cad5b096925642ca359a6d6f0562a
        // cf) https://github.com/Kerollmops/heed/blob/b235e9c3e9984737c967b5de1014b48f125dc28b/heed/src/env.rs#L359
        // cf) https://github.com/danburkert/lmdb-rs/blob/c64f0b4f8280190525a6430388e714739f18976b/lmdb-sys/src/ffi.rs#L93
        let db = env.create_database::<Str, SerdeJson<T>>(None)?;
        Ok(Index{
            env,
            main: db,
        })
    }
}