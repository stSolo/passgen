pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "passgen", about = "a passgen struct")]
pub struct Cli {
    #[structopt(short, long, default_value = "16")]
    length: u8
}

impl Cli {
    pub fn length(&self) -> u8 {
        self.length
    }
}