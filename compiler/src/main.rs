use std::env;

use narwhal::{cir::Cir, index::Index};
use yansi::Condition;

fn main() {
    yansi::whenever(Condition::TTY_AND_COLOR);

    let index = Index::load(env::current_dir().unwrap()).unwrap();
    dbg!(&index);

    let cir = Cir::new(&index);
    dbg!(&cir);
}
