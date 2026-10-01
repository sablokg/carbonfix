use smartcore::linalg::basic::matrix::DenseMatrix;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn predict(pathfile: &str) -> Result<DenseMatrix<f64>, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut value_entire_string: Vec<(String, Vec<String>)> = Vec::new();
    let mut years: HashSet<String> = HashSet::new();

    for i in fileread.lines() {
        let line = i.expect("file not present");
        if line.starts_with("#") {
            continue;
        }
        if !line.starts_with("#") {
            let linevec = line.split("\t").collect::<Vec<_>>();
            years.insert(linevec[0].to_string());
            value_entire_string.push((
                linevec[0].to_string(),
                linevec.iter().map(|x| x.to_string()).collect::<Vec<_>>(),
            ));
        }
    }

    let mut tensorvec: Vec<Vec<f64>> = Vec::new();

    for i in years.iter() {
        for val in value_entire_string.iter() {
            if *i == val.0 {
                let mut value_3: Vec<f64> = Vec::new();
                let mut value_4: Vec<f64> = Vec::new();
                let mut value_6: Vec<f64> = Vec::new();
                value_3.push(val.1[3].parse::<f64>().unwrap());
                value_4.push(val.1[4].parse::<f64>().unwrap());
                value_6.push(val.1[6].parse::<f64>().unwrap());
                let value3_array = ndarray::Array1::from_vec(value_3);
                let value4_array = ndarray::Array1::from_vec(value_4);
                let value6_array = ndarray::Array1::from_vec(value_6);
                let unifiedvalue = value3_array + value4_array + value6_array;
                let finalarray = unifiedvalue.to_vec();
                tensorvec.push(finalarray);
            }
        }
    }

    let densematrix: DenseMatrix<f64> = DenseMatrix::from_2d_vec(&tensorvec).unwrap();

    Ok(densematrix)
}
