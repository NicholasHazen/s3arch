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