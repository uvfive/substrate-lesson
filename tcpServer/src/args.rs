use structopt::StructOpt;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo_server", version = "1.2.0", author = "Vz", about = "An example of StructOpt usage.")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,
    /// verbosity level
    #[structopt(short = "v", long, parse(from_occurrences))]
    verbose: u32,
    /// Set speed
    #[structopt(short = "s", long = "speed", default_value = "42")]
    speed: f64,
    /// Input file
    #[structopt(parse(from_str))]
    input: Option<String>,
    /// admin_level to consider
    #[structopt(short, long)]
    level: Vec<String>,
    /// Output file
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
    // listen ip address
    #[structopt(long, default_value = "127.0.0.1")]
    ip: String,

    #[structopt(long, default_value = "9123")]
    port: u32,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[structopt(short = "c", long)]
    nb_cars: Option<i32>,
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "self fmt: (verbose:{}, debug: {}, level:{:?}, ip:{}, port:{})", self.verbose, self.debug, self.level, self.ip, self.port)
    }
}

impl Opt {
    pub fn hostname(&self) -> String {
        return format!("{}:{:?}", self.ip, self.port);
    }

    pub fn parse_args() -> Self {
        return Opt::from_args();
    }
}
