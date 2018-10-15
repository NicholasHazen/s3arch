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

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    s3arch::run(matches);
//
//    let client = S3Client::new(Region::UsEast1);

//    let mut list_request = ListObjectsRequest::default();
//    list_request.bucket = String::from("pokki-analytics-raw");
//
//    let list_result = client.list_objects(list_request).sync().unwrap();
//
//    for object in list_result.contents.unwrap().iter() {
//        println!("Object Key: {}", object.key.clone().unwrap());
//        println!("Size: {}", object.size.clone().unwrap());
//    }
//
//    // Get Object
//    let mut object_request = GetObjectRequest::default();
//    object_request.bucket = String::from("pokki-analytics-raw");
//    object_request.key = String::from("nginx/analytics_capture-access_log.2018100516-1538755201.log-4pslTonUOb4G.gz");
//
//    let object_result = client.get_object(object_request).sync().unwrap();
//
//
//    let stream = object_result.body.unwrap();
//    let body = stream.concat2().wait().unwrap();
//
//    let b = body.as_slice();
//
//    let mut d = GzDecoder::new(b);
//    let mut s = String::new();
//    d.read_to_string(&mut s).unwrap();
//
//    println!("Body: {}", s);
}
