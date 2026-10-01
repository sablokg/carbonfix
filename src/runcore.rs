use crate::logreg::cor2predict;
use crate::predict::predict;
use smartcore::ensemble::random_forest_regressor::RandomForestRegressor;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;
use smartcore::neighbors::knn_regressor::KNNRegressor;
use smartcore::xgboost::{XGRegressor, XGRegressorParameters};
use std::error::Error;
use std::fs::File;
use std::io::Write;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn runmodel(pathfile: &str, pathfile2: &str) -> Result<String, Box<dyn Error>> {
    let valueunpack = cor2predict(pathfile).unwrap();
    let densematrix = valueunpack.0;
    let inputvec = valueunpack.1;
    let vectorunpack = train_test_split(&densematrix, &inputvec, 0.8, true, Some(11));
    let logisticput =
        LogisticRegression::fit(&vectorunpack.0, &vectorunpack.2, Default::default()).unwrap();
    let logisticpredict = logisticput.predict(&vectorunpack.1).unwrap();
    let accuracymodel_log = accuracy(&vectorunpack.3, &logisticpredict);
    println!(
        "The accuracy of the predicted model is {}",
        accuracymodel_log
    );

    let predictvalue = predict(pathfile2).expect("file not present");

    let predictlog = logisticput.predict(&predictvalue).unwrap();

    let randomereg =
        RandomForestRegressor::fit(&vectorunpack.0, &vectorunpack.2, Default::default()).unwrap();
    let randompredict = randomereg.predict(&vectorunpack.1).unwrap();
    let random_accuracy = accuracy(&vectorunpack.3, &randompredict);
    println!(
        "The model accuracy for the random accuracy is {}",
        random_accuracy
    );

    let predictrandom = randomereg.predict(&predictvalue).unwrap();

    let ensemblreg =
        KNNRegressor::fit(&vectorunpack.0, &vectorunpack.2, Default::default()).unwrap();
    let ensemblpredict = ensemblreg.predict(&vectorunpack.1).unwrap();
    let accuracy_knn = accuracy(&vectorunpack.3, &ensemblpredict);
    println!("The value for the KNN predict value is {}", accuracy_knn);

    let ensemblpredict = ensemblreg.predict(&predictvalue).unwrap();

    let xgbparameter = XGRegressorParameters::default()
        .with_n_estimators(50)
        .with_max_depth(10)
        .with_learning_rate(0.1);

    let xgfit = XGRegressor::fit(&vectorunpack.0, &vectorunpack.2, xgbparameter).unwrap();
    let xgpredict = xgfit.predict(&vectorunpack.1).unwrap();
    let xgpredict_value = accuracy(
        &vectorunpack.3,
        &xgpredict
            .iter()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>(),
    );
    println!("The value for the prediction is {}", xgpredict_value);

    let xgpredict = xgfit.predict(&predictvalue).unwrap();

    let mut filevalue1 = File::create("predictlog.txt").expect("file not present");
    for i in predictlog.iter() {
        writeln!(filevalue1, "{}\n", i).expect("file not present");
    }

    let mut filevalue2 = File::create("randompredict.txt").expect("file not present");
    for i in predictrandom.iter() {
        writeln!(filevalue2, "{}\n", i).expect("file not present");
    }

    let mut filevalue3 = File::create("knnpredict.txt").expect("file not present");
    for i in ensemblpredict.iter() {
        writeln!(filevalue3, "{}\n", i).expect("file not present");
    }

    let mut filevalue4 = File::create("xgboostpredict.txt").expect("file not present");
    for i in xgpredict.iter() {
        writeln!(filevalue4, "{}\n", i).expect("file not present");
    }

    Ok("The model run has finished".to_string())
}
