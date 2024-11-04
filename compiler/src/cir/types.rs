use cranelift_entity::PrimaryMap;

use crate::index::GlobalId;

// TODO: interning types
// TODO: store TypeInfo or TypeVar in CIR?

#[derive(Debug, Default, Clone, Hash)]
pub struct TypeContext {
    vars: PrimaryMap<TypeVar, TypeInfo>,
}

impl TypeContext {
    pub fn new() -> Self {
        TypeContext::default()
    }

    pub fn insert(&mut self, info: TypeInfo) -> TypeVar {
        let var = TypeVar(self.vars.len() as u32);
        self.vars.push(info);
        var
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TypeVar(u32);
cranelift_entity::entity_impl!(TypeVar);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TypeInfo {
    Unknown,
    Equal(TypeVar),
    TypeDef(GlobalId),
    Function {
        params: Vec<TypeVar>,
        return_ty: TypeVar,
    },
    Bool,
    Int,
    Float,
    Char,
    String,
    Void,
}
