use axonml::nn::prelude::*;
use axonml::optim::{Adam, Optimizer};
use axonml::prelude::Variable;
use axonml::tensor::Tensor;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

type REURNVEC = (Vec<Vec<f64>>, Vec<i32>);

pub fn axonml(pathfile: &str) -> Result<String, Box<dyn Error>> {
    let preparedata = preparetensor(pathfile).unwrap();
    let featurestensor = Tensor::from_vec(
        preparedata
            .0
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.to_string().parse::<f32>().unwrap())
            .collect::<Vec<_>>(),
        &[preparedata.0[0].len()],
    )
    .unwrap();
    let labeltensor = Tensor::from_vec(
        preparedata
            .1
            .clone()
            .iter()
            .map(|x| x.to_string().parse::<f32>().unwrap())
            .collect::<Vec<_>>(),
        &[preparedata.1.clone().len()],
    )
    .unwrap();

    let model = Sequential::new()
        .add(Linear::new(preparedata.0.len(), 16))
        .add(ReLU)
        .add(Linear::new(16, 1));

    let mseloss = MSELoss::new();
    let mut optimizer = Adam::new(model.parameters(), 0.1);

    for i in 0..100 {
        let xtrain = Variable::new(featurestensor.clone(), false);
        let ytrain = Variable::new(labeltensor.clone(), false);

        let modelforward = model.forward(&xtrain);
        let loss = mseloss.compute(&modelforward, &ytrain);
        optimizer.zero_grad();
        loss.backward();
        optimizer.step();

        if i % 10usize == 0 {
            println!("Epoch:{}", i);
        }
    }

    Ok("axonml has finished".to_string())
}

pub fn preparetensor(pathfile: &str) -> Result<REURNVEC, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);
    let mut featuresvec: Vec<Vec<f64>> = Vec::new();
    let mut labelsvec: Vec<i32> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("file not present");
        let linevec = line.split(",").collect::<Vec<_>>();
        let featuressplit = linevec[0..linevec.len() - 1]
            .to_vec()
            .iter()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        let label: i32 = linevec[linevec.len() - 1..linevec.len()].to_vec()[0]
            .parse::<i32>()
            .unwrap();
        featuresvec.push(featuressplit);
        labelsvec.push(label);
    }

    let unifiedvec: (Vec<Vec<f64>>, Vec<i32>) = (featuresvec, labelsvec);
    Ok(unifiedvec)
}
