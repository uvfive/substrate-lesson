use structopt::StructOpt;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo_server", version = "1.2.0", author = "Vz", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,
    /// verbosity level
    #[structopt(short = "v", long, parse(from_occurrences))]
    pub verbose: u32,
    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    pub speed: f64,
    /// admin_level to consider
    #[structopt(short, long)]
    pub level: Vec<String>,
    /// Input file
    #[structopt(parse(from_str))]
    pub input: Option<String>,
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    pub output: Option<PathBuf>,
    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    pub nb_cars: Option<i32>,
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "self fmt::Display (verbose:{}, debug: {}, speed:{}, level:{:?}, output:{:?}, nb_cars: {:?})", self.verbose, self.debug, self.speed, self.level, self.output, self.nb_cars)
    }
}

impl Opt {
    pub fn parse_args() -> Self {
        return Opt::from_args();
    }
}
