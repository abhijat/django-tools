use structopt::StructOpt;

use project_checker as pc;

#[derive(StructOpt, Debug)]
#[structopt(version = "0.1.0", author = "Abhijat Malviya")]
pub struct Opts {
    #[structopt(required = true, short = "r", long)]
    pub source_roots: Vec<String>,

    #[structopt(required = true, help = "the model to search receivers for", short, long)]
    pub subject: String,
}


fn main() {
    let opts: Opts = Opts::from_args();
    let stage_receivers = pc::find_receivers_in_paths(
        &pc::to_refs(&opts.source_roots),
        &opts.subject,
    ).unwrap();
    for (stage, receivers) in &stage_receivers {
        println!("{}", stage.to_string());
        for receiver in receivers {
            println!(" {}", receiver.to_string());
        }
    }
}