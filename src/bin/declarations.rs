use structopt::StructOpt;

use project_checker::{collect_source_files, get_declarations, Opts, to_refs};

fn main() {
    let opts: Opts = Opts::from_args();
    let sources = collect_source_files(
        &to_refs(&opts.source_roots),
        ".py"
    ).unwrap();
    let declarations = get_declarations(&sources);
    eprintln!("declarations = {:?}", declarations);
}