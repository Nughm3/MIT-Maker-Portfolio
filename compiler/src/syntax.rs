pub use cstree::interning::TokenKey;
use cstree::text::TextRange;

pub use self::generated::{ast, kind::SyntaxKind, token};

#[rustfmt::skip]
mod generated;

pub type SyntaxNode = cstree::syntax::SyntaxNode<SyntaxKind>;
pub type SyntaxToken = cstree::syntax::SyntaxToken<SyntaxKind>;
pub type SyntaxElement = cstree::syntax::SyntaxElement<SyntaxKind>;
pub type SyntaxElementRef<'a> = cstree::syntax::SyntaxElementRef<'a, SyntaxKind>;

pub trait AstInternedToken: AstElement {
    fn text_key(&self) -> TokenKey;
}

pub trait AstElement: Sized {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(elem: SyntaxElement) -> Option<Self>;
    fn range(&self) -> TextRange;
}
