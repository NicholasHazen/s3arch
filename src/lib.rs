extern crate clap;
extern crate rusoto_core;
extern crate rusoto_s3;

use clap::ArgMatches;
use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, ListObjectsRequest, GetObjectRequest, StreamingBody};

pub fn run(matches: ArgMatches) -> Result<(), &'static str> {
    let bucket_name = matches.value_of("bucket").unwrap();
    let term = matches.value_of("term").unwrap();

    println!("Bucket: {}", bucket_name);
    println!("Term: {}", term);

    let client = S3Client::new(Region::UsEast1);

    let mut keys = get_object_list(client, bucket_name).unwrap();
    keys.retain(|ref input| filter_object(input, &String::from("nginx")));

    println!("Keys: {:?}", keys);

    Ok(())
}

pub fn get_object_list(client: S3Client, bucket: &str) -> Result<Vec<String>, &'static str> {
    let mut list_request = ListObjectsRequest::default();
    list_request.bucket = bucket.to_string();

    let list_result = client.list_objects(list_request).sync().unwrap();

    let mut object_keys = Vec::new();

    for object in list_result.contents.unwrap().iter() {
        object_keys.push(object.key.clone().unwrap());
    }

    Ok(object_keys)
}

pub fn filter_object(name: &String, filter: &String) -> bool {
    name.contains(filter)
}

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