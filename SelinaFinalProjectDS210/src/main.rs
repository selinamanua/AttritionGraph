#![allow(dead_code)]
use std::{error::Error,io,io::prelude::*};
use serde::Deserialize;
mod graph;
mod graph_test;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
#[allow(dead_code,non_snake_case)]
pub struct Record {
    Age: Option<i32>,
    Attrition: String,
    BusinessTravel: String,
    DailyRate: Option<i32>, 
    Department: String,
    DistanceFromHome: Option<i32>,
    Education: Option<i32>,
    EducationField: String,
    EmployeeCount: Option<i32>,
    EmployeeNumber: Option<i32>,
    EnvironmentSatisfaction: Option<i32>,
    Gender: String,
    HourlyRate: Option<i32>,
    JobInvolvement: Option<i32>,
    JobLevel: Option<i32>,
    JobRole: String,
    JobSatisfaction: Option<i32>,
    MaritalStatus: String,
    MonthlyIncome: Option<i32>,
    MonthlyRate: Option<i32>,
    NumCompaniesWorked: Option<i32>,
    Over18: String,
    PercentSalaryHike: Option<i32>,
    PerformanceRating: Option<i32>,
    RelationshipSatisfaction: Option<i32>,
    StandardHours: Option<i32>,
    StockOptionLevel: Option<i32>,
    TotalWorkingYears: Option<i32>,
    TrainingTimesLastYear: Option<i32>,
    WorkLifeBalance: Option<i32>,
    YearsAtCompany: Option<i32>,
    YearsInCurrentRole: Option<i32>,
    YearsSinceLastPromotion: Option<i32>,
    YearsWithCurrManager: Option<i32>,
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("./src/dataset.csv")?;
    let mut records = Vec::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("Node to search: ");
    stdout.flush().unwrap();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let num: usize = input.trim().parse().unwrap();
    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
        // println!("{:?}", record);
    }
    // println!("{:?}", records[0]);
    let employeeGraph = graph::graph::new(&records);
    // println!("{:?}", employeeGraph.GraphMatrix);
    if let Some(Distance) = graph::graph::graph_analysis(&employeeGraph, num) {
        println!("Distance between nodes: {}", Distance);
    }
    Ok(())
}
