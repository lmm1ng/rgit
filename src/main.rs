use std::env;
use rgit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(git_config) = rgit::find_outer_config() {
        rgit::apply_config(git_config).ok();
    };

    rgit::exec_git(&args).unwrap();
}
