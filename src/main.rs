#[macro_use]
extern crate clap;
extern crate rusoto_credential;
extern crate rusoto_core;
extern crate rusoto_s3;
extern crate s3arch;
extern crate futures;
extern crate flate2;

use std::io::prelude::*;
use clap::App;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, ListObjectsRequest, GetObjectRequest, StreamingBody};
use futures::{Future, Stream};
use flate2::read::GzDecoder;
use std::str;

//TODO MVP - Add documents
//TODO MVP - Iterate through multiple object lists
//TODO MVP - Create file streaming functions
//TODO MVP - Create Search funtionality
//TODO MVP - Create Display functionality
//TODO MVP - Cleanup error handling
//TODO MVP - First round of refactoing
//TODO Experiment - Accecpt javascript for filter expression

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    s3arch::run(matches);
}
