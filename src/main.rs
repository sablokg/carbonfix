mod args;
use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
use figlet_rs::FIGfont;
mod logreg;
mod runcore;
use crate::runcore::runmodel;
mod axon;
mod predict;
use crate::axon::axonml;
mod align;
mod filestruct;
use crate::filestruct::FileName;

/*
Gaurav Sablok
gsablok@proton.me
*/

fn main() {
    let fontgenerate = FIGfont::standard().unwrap();
    let repgenerate = fontgenerate.convert("carbonfix");
    println!("{}", repgenerate.unwrap());

    let args = CommandParse::parse();
    match &args.command {
        Commands::LogisticRegressor {
            filepath,
            prediction,
            thread,
        } => {
            let n_threads = thread.parse::<usize>().expect("thread must be a number");
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let runmodel = runmodel(filepath, prediction).unwrap();
                println!("The qsar modelling has finished: {}", runmodel);
            });
        }
        Commands::Neural { pathfile, threads } => {
            let n_threads = threads.parse::<usize>().unwrap();
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("threads failed");
            pool.install(|| {
                let value = axonml(pathfile).unwrap();
                println!("The axon neural has finished: {}", value);
            })
        }
        Commands::AlignmentFiles {
            filepath,
            windowlength,
            thread,
        } => {
            let n_threads = thread.parse::<usize>().expect("thread must be a number");
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build()
                .expect("failed to create thread pool");
            pool.install(|| {
                let filepathunwrap = FileName {
                    pathfile: filepath.clone(),
                };
                let value = filepathunwrap.alignment(windowlength).unwrap();
                println!("The qsar modelling has finished: {:?}", value);
            });
        }
    }
}
