use crate::filestruct::Fasta;
use crate::filestruct::FileName;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

impl FileName {
    pub fn alignment(&self, windowlength: &str) -> Result<String, Box<dyn Error>> {
        let fileopen = File::open(self.pathfile.clone()).expect("file not present");
        let fileread = BufReader::new(fileopen);

        let mut filepathvec: Vec<Fasta> = Vec::new();
        let mut header: Vec<String> = Vec::new();
        let mut sequence: Vec<String> = Vec::new();

        for i in fileread.lines() {
            let fileline = i.expect("file not present");
            if fileline.starts_with(">") {
                header.push(fileline.replace(">", ""));
            }
            if !fileline.starts_with(">") {
                sequence.push(fileline);
            }
        }

        for i in 0..header.len() {
            filepathvec.push(Fasta {
                idname: header[i].clone(),
                nameseq: sequence[i].clone(),
            });
        }

        let windowlength_iter = windowlength.parse::<usize>().unwrap();
        let mut vecstring: Vec<Vec<&str>> = Vec::new();
        for i in sequence.iter() {
            let stringval = i
                .as_bytes()
                .chunks_exact(windowlength_iter)
                .collect::<Vec<_>>();
            let vecvalue = stringval
                .iter()
                .map(|x| str::from_utf8(x).unwrap())
                .collect::<Vec<_>>();
            vecstring.push(vecvalue);
        }

        let rowcount = vecstring.len();
        let colcount = vecstring[0].len();
        let mut resultvec = Vec::new();
        for val in 0..colcount {
            let mut rowval = Vec::new();
            for i in 0..rowcount {
                rowval.push(vecstring[i][val].to_string());
            }
            resultvec.push(rowval);
        }

        let mut valuevec: Vec<(String, usize)> = Vec::new();
        for i in 0..resultvec.len() {
            let valuinput = resultvec[i][0].clone();
            let restvec = resultvec[1..resultvec.len()]
                .to_vec()
                .iter()
                .cloned()
                .flatten()
                .collect::<Vec<_>>();
            let value = similarity(valuinput.clone(), restvec).unwrap();
            valuevec.push((valuinput.clone(), value));
        }

        println!("{:?}", resultvec);

        Ok("path string has been written".to_string())
    }
}

pub fn similarity(value1: String, value2: Vec<String>) -> Result<usize, Box<dyn Error>> {
    let value1 = value1.chars().collect::<Vec<_>>();
    let mut value2_map: Vec<Vec<char>> = Vec::new();
    for i in value2.iter() {
        let val = i.chars().collect::<Vec<_>>();
        value2_map.push(val);
    }
    let mut valuetuple: Vec<usize> = Vec::new();
    for i in 0..value1.len() {
        for val in 0..value2_map.len() {
            if value1[i] != value2_map[val][i] {
                valuetuple.push(1usize);
            }
        }
    }

    let finalvalue: usize = valuetuple.iter().sum();
    println!("{:?}", finalvalue);
    Ok(finalvalue)
}
