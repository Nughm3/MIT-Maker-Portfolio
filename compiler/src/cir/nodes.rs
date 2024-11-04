use cstree::interning::TokenKey;
use ordered_float::NotNan;

use super::types::TypeVar;
use crate::index::GlobalId;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LocalId(u32);
cranelift_entity::entity_impl!(LocalId);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExprId(u32);
cranelift_entity::entity_impl!(ExprId);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Def {
    Type(TypeDef),
    Function(Function),
    Constant(Binding),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Local {
    Function(Function),
    Binding(Binding),
    Param(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TypeDef {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Field {
    pub name: TokenKey,
    pub ty: TypeVar,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Function {
    pub params: Vec<(TokenKey, TypeVar)>,
    pub return_ty: TypeVar,
    pub body: Block,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Binding {
    pub name: TokenKey,
    pub kind: BindingKind,
    pub value: ExprId,
    pub ty: TypeVar,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BindingKind {
    Constant,
    Variable,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Block {
    pub exprs: Vec<ExprId>,
}

// TODO: deduplicate simple Exprs like break/continue/atom
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Expr {
    Block(Block),
    Variable(LocalId),
    Loop(Block),
    Break,
    Continue,
    Return(Option<ExprId>),
    Decision(Decision),
    Prefix {
        op: PrefixOp,
        expr: ExprId,
    },
    Infix {
        lhs: ExprId,
        op: InfixOp,
        rhs: ExprId,
    },
    Assign {
        lhs: ExprId,
        rhs: ExprId,
    },
    Call {
        function: ExprId,
        args: Vec<ExprId>,
    },
    Access {
        expr: ExprId,
        name: TokenKey,
    },
    Closure, // TODO
    Global(GlobalId),
    Local(LocalId),
    Atom(Atom), // TODO: deduplicate non-String atoms
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Decision {
    pub conditions: Vec<ExprId>,
    pub branches: Vec<Block>,
    pub default: Option<Block>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PrefixOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InfixOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Atom {
    Bool(bool),
    Int(u64),
    Float(NotNan<f64>),
    Char(char),
    // raw string token, including quotes and escape characters
    String(TokenKey),
}
