use structopt::StructOpt;

use project_checker::{collect_source_files, get_declarations, to_refs, Opts};

fn main() {
    let opts: Opts = Opts::from_args();
    let sources = collect_source_files(&to_refs(&opts.source_roots), ".py");
    let declarations = get_declarations(&sources);
    eprintln!("declarations = {:?}", declarations);
}
