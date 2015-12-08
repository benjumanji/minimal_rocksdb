extern crate rocksdb;

use rocksdb::{DB, DBVector, MergeOperands, Options, Writable, WriteBatch};
use rocksdb::rocksdb_ffi::DBCFHandle;

fn create_fresh(opts: &Options, path: &str) -> Result<DB, String> {
    let mut db = try!(DB::open(&opts, path));
    try!(db.create_cf("cf", &opts));
    Ok(db)
}

fn main() {
    let mut opts = Options::new();
    opts.create_if_missing(true);

    let cf = "cf";
    let cfs = ["cf"];
    let path = "/tmp/foo";

    let db = DB::open_cf(&opts, path, &cfs).or(create_fresh(&opts, path)).unwrap();
    let intern_handle = *db.cf_handle(cf).ok_or("boom!").unwrap();
    let iter = db.iterator_cf(cf).unwrap();
}
