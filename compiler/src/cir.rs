use cranelift_entity::{PrimaryMap, SecondaryMap};
use nodes::*;
use types::TypeContext;

use crate::index::GlobalId;

mod construct;
pub mod nodes;
mod resolve;
pub mod types;

#[derive(Debug, Default, Clone)]
pub struct Cir {
    tcx: TypeContext,
    globals: SecondaryMap<GlobalId, Option<Def>>,
    locals: PrimaryMap<LocalId, Option<Local>>,
    exprs: PrimaryMap<ExprId, Expr>,
}
