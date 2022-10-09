use std::env;
use rgit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let git_config = rgit::find_outer_config();
    match git_config {
        Ok(config) => rgit::apply_config(config),
        Err(_) => ()
    };
    rgit::exec_git(&args).unwrap();
}
