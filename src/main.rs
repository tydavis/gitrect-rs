use std::collections::HashMap;
use std::path::Path;

struct Repo {
    path: Path,
    remotes: HashMap<String, url::URL>,
}

fn main() {
    println!("Hello, world!");
}
