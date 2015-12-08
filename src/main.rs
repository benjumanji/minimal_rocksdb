extern crate rocksdb;

use rocksdb::{DB, DBVector, MergeOperands, Options, Writable, WriteBatch};
use rocksdb::rocksdb_ffi::DBCFHandle;

fn get_or_create(opts: &Options, db: &mut DB) -> Result<DBCFHandle, String> {
    db.cf_handle("cf")
        .map(|x| -> Result<DBCFHandle, String> { Ok(*x) })
        .unwrap_or_else(|| { db.create_cf("cf", &opts) })
}

fn main() {
    let mut opts = Options::new();
    opts.create_if_missing(true);

    let cf = "cf";
    let cfs = ["cf"];
    let path = "/tmp/foo";

    let mut db = DB::open_cf(&opts, path, &cfs).or(DB::open(&opts, path)).unwrap();
    let intern_handle = get_or_create(&opts, &mut db);
    let iter = db.iterator_cf(cf).unwrap();
}
