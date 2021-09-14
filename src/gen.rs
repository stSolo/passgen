use cli::StructOpt;
use cli::Cli;
use rand::Rng;
mod cli;
mod chrset;

pub fn start() -> Cli{
    cli::Cli::from_args()
}

pub fn gen(cli: Cli) -> String{
    let len = cli.length();
    let charset: &[u8] = &[chrset::AZ, chrset::az, chrset::nums, chrset::symbs].concat();
    let mut rng = rand::thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    password
}