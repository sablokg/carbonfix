use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "CO2Model",
    version = "1.0",
    about = "Regression Modelling of CO2
       ************************************************
       Author Gaurav Sablok,
       Email: gsablok@proton.me
      ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// CO2 Sheet
    LogisticRegressor {
        /// path to the file
        filepath: String,
        /// path to the prediction file
        prediction: String,
        /// threads for the analysis
        thread: String,
    },
    /// CO2 Neural Axon Logistic regressor
    Neural {
        /// path to the file
        pathfile: String,
        /// threads for the analysis
        threads: String,
    },
    /// fasta file for the alignment
    AlignmentFiles {
        /// path to the file
        filepath: String,
        /// windowlength
        windowlength: String,
        /// threads for the analysis
        thread: String,
    },
}
