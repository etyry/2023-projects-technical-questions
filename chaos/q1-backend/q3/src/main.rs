// q3
// In `student.psv` there are some fake student datas from UNSW CSE (no doxx!). In each row, the fields from left to right are
//
// - UNSW Course Code
// - UNSW Student Number
// - Name
// - UNSW Program
// - UNSW Plan
// - WAM
// - UNSW Session
// - Birthdate
// - Sex
// 
// Write a Rust program to find the course which has the highest average student WAM.

use csv::Error;
use std::collections::HashMap;

// record type
type Record = (String, f64, String, String, String, f64, String, f64, String);

fn main() -> Result<(), Error> {
    let mut courses = HashMap::new();
    let mut students = HashMap::new();
    
    // create a psv parser that reads data from given path
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_path("../student.psv")?;

    // loop over each record and add the total course wam and student num to hashmaps
    for record in reader.deserialize() {
        let record: Record = record?;
        courses.entry(record.0.to_owned())
            .and_modify(|v| *v += record.5)
            .or_insert(record.5);
        students.entry(record.0.to_owned())
            .and_modify(|v| *v += 1.0)
            .or_insert(1.0_f64);
    }

    // find highest average course and wam
    let mut highest_wam = 0.0_f64;
    let mut wam;
    let mut highest_course = String::new();
    for key in courses.keys() {
        wam = courses.get(key).unwrap() / students.get(key).unwrap();
        if highest_wam < wam {
            highest_wam = wam;
            highest_course = (&key).to_string();
        }
    }

    println!("{:?} has the highest average student WAM of {:?}", highest_course, highest_wam);

    Ok(())
}

// Use hashmap to store course codes and total wam for the course