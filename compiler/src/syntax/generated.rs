pub mod kind {
    use logos::Logos;
    use cstree::{Syntax, text::{TextRange, TextSize}};
    #[repr(u32)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Logos, Syntax)]
    pub enum SyntaxKind {
        #[token("import")]
        #[static_text("import")]
        ImportKw,
        #[token(";")]
        #[static_text(";")]
        Semicolon,
        #[token("type")]
        #[static_text("type")]
        TypeKw,
        #[regex("[a-zA-Z_][a-zA-Z\\d_]*")]
        Ident,
        #[token("(")]
        #[static_text("(")]
        LeftParen,
        #[token(")")]
        #[static_text(")")]
        RightParen,
        #[token(":")]
        #[static_text(":")]
        Colon,
        #[token(",")]
        #[static_text(",")]
        Comma,
        #[token("fn")]
        #[static_text("fn")]
        FnKw,
        #[token("->")]
        #[static_text("->")]
        Arrow,
        #[token("const")]
        #[static_text("const")]
        ConstKw,
        #[token("=")]
        #[static_text("=")]
        Equals,
        #[token("{")]
        #[static_text("{")]
        LeftBrace,
        #[token("}")]
        #[static_text("}")]
        RightBrace,
        #[token("let")]
        #[static_text("let")]
        LetKw,
        #[token("loop")]
        #[static_text("loop")]
        LoopKw,
        #[token("while")]
        #[static_text("while")]
        WhileKw,
        #[token("break")]
        #[static_text("break")]
        BreakKw,
        #[token("continue")]
        #[static_text("continue")]
        ContinueKw,
        #[token("return")]
        #[static_text("return")]
        ReturnKw,
        #[token("if")]
        #[static_text("if")]
        IfKw,
        #[token("else")]
        #[static_text("else")]
        ElseKw,
        #[token("+")]
        #[static_text("+")]
        Plus,
        #[token("-")]
        #[static_text("-")]
        Minus,
        #[token("*")]
        #[static_text("*")]
        Star,
        #[token("/")]
        #[static_text("/")]
        Slash,
        #[token("%")]
        #[static_text("%")]
        Percent,
        #[token("==")]
        #[static_text("==")]
        Eq,
        #[token("!=")]
        #[static_text("!=")]
        Ne,
        #[token("<")]
        #[static_text("<")]
        Lt,
        #[token("<=")]
        #[static_text("<=")]
        Le,
        #[token(">")]
        #[static_text(">")]
        Gt,
        #[token(">=")]
        #[static_text(">=")]
        Ge,
        #[token("and")]
        #[static_text("and")]
        AndKw,
        #[token("or")]
        #[static_text("or")]
        OrKw,
        #[token("+=")]
        #[static_text("+=")]
        PlusEquals,
        #[token("-=")]
        #[static_text("-=")]
        MinusEquals,
        #[token("*=")]
        #[static_text("*=")]
        StarEquals,
        #[token("/=")]
        #[static_text("/=")]
        SlashEquals,
        #[token("%=")]
        #[static_text("%=")]
        PercentEquals,
        #[token("not")]
        #[static_text("not")]
        NotKw,
        #[token("package")]
        #[static_text("package")]
        PackageKw,
        #[token(".")]
        #[static_text(".")]
        Dot,
        #[token("true")]
        #[static_text("true")]
        True,
        #[token("false")]
        #[static_text("false")]
        False,
        #[regex("\\d[\\d_]*|0x[\\da-fA-F][\\da-fA-F_]*|0b[01][01_]*")]
        Int,
        #[regex(
            "\\d[\\d_]*(\\.(\\d[\\d_]*)?|\\.\\d[\\d_]*[eE][+-]?\\d[\\d_]*|[eE][+-]?\\d[\\d_]*)"
        )]
        Float,
        #[regex("'([^']|\\\\[\\\\nrt0'])'")]
        Char,
        #[regex("\"([^\"]|\\\\[\\\\nrt0\"])*\"")]
        String,
        File,
        Import,
        TypeDef,
        Function,
        Constant,
        Path,
        AdtField,
        TypeFunction,
        TypeList,
        TypeAscription,
        Signature,
        Block,
        ParamList,
        StmtExpr,
        StmtLet,
        StmtLoop,
        StmtWhile,
        StmtBreak,
        StmtContinue,
        StmtReturn,
        ExprIf,
        ExprParen,
        ExprPrefix,
        ExprInfix,
        ExprAssign,
        ExprCall,
        ExprClosure,
        ExprList,
        PathComponent,
        #[regex(r"[\n\r]+")]
        Newlines,
        #[regex(r"[ \t]+")]
        Whitespace,
        #[regex("//.*")]
        Comment,
        ErrorToken,
        ErrorTree,
        ParserTest,
        Eof,
    }
    impl SyntaxKind {
        pub fn lexer(input: &str) -> impl Iterator<Item = (SyntaxKind, TextRange)> + '_ {
            logos::Logos::lexer(input)
                .spanned()
                .map(move |(token, range)| {
                    (
                        token.unwrap_or(SyntaxKind::ErrorToken),
                        TextRange::new(
                            TextSize::new(range.start as u32),
                            TextSize::new(range.end as u32),
                        ),
                    )
                })
        }
    }
}
pub mod token {
    use std::fmt;
    use crate::syntax::*;
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ImportKw(SyntaxToken);
    impl fmt::Debug for ImportKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ImportKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ImportKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Semicolon(SyntaxToken);
    impl fmt::Debug for Semicolon {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Semicolon {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Semicolon
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct TypeKw(SyntaxToken);
    impl fmt::Debug for TypeKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for TypeKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::TypeKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Ident(SyntaxToken);
    impl fmt::Debug for Ident {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstInternedToken for Ident {
        fn text_key(&self) -> TokenKey {
            self.0.text_key().unwrap()
        }
    }
    impl AstElement for Ident {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Ident
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct LeftParen(SyntaxToken);
    impl fmt::Debug for LeftParen {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for LeftParen {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::LeftParen
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct RightParen(SyntaxToken);
    impl fmt::Debug for RightParen {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for RightParen {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::RightParen
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Colon(SyntaxToken);
    impl fmt::Debug for Colon {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Colon {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Colon
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Comma(SyntaxToken);
    impl fmt::Debug for Comma {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Comma {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Comma
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct FnKw(SyntaxToken);
    impl fmt::Debug for FnKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for FnKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::FnKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Arrow(SyntaxToken);
    impl fmt::Debug for Arrow {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Arrow {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Arrow
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ConstKw(SyntaxToken);
    impl fmt::Debug for ConstKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ConstKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ConstKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Equals(SyntaxToken);
    impl fmt::Debug for Equals {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Equals {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Equals
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct LeftBrace(SyntaxToken);
    impl fmt::Debug for LeftBrace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for LeftBrace {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::LeftBrace
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct RightBrace(SyntaxToken);
    impl fmt::Debug for RightBrace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for RightBrace {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::RightBrace
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct LetKw(SyntaxToken);
    impl fmt::Debug for LetKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for LetKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::LetKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct LoopKw(SyntaxToken);
    impl fmt::Debug for LoopKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for LoopKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::LoopKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct WhileKw(SyntaxToken);
    impl fmt::Debug for WhileKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for WhileKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::WhileKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct BreakKw(SyntaxToken);
    impl fmt::Debug for BreakKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for BreakKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::BreakKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ContinueKw(SyntaxToken);
    impl fmt::Debug for ContinueKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ContinueKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ContinueKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ReturnKw(SyntaxToken);
    impl fmt::Debug for ReturnKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ReturnKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ReturnKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct IfKw(SyntaxToken);
    impl fmt::Debug for IfKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for IfKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::IfKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ElseKw(SyntaxToken);
    impl fmt::Debug for ElseKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ElseKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ElseKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Plus(SyntaxToken);
    impl fmt::Debug for Plus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Plus {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Plus
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Minus(SyntaxToken);
    impl fmt::Debug for Minus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Minus {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Minus
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Star(SyntaxToken);
    impl fmt::Debug for Star {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Star {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Star
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Slash(SyntaxToken);
    impl fmt::Debug for Slash {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Slash {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Slash
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Percent(SyntaxToken);
    impl fmt::Debug for Percent {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Percent {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Percent
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Eq(SyntaxToken);
    impl fmt::Debug for Eq {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Eq {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Eq
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Ne(SyntaxToken);
    impl fmt::Debug for Ne {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Ne {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Ne
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Lt(SyntaxToken);
    impl fmt::Debug for Lt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Lt {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Lt
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Le(SyntaxToken);
    impl fmt::Debug for Le {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Le {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Le
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Gt(SyntaxToken);
    impl fmt::Debug for Gt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Gt {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Gt
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Ge(SyntaxToken);
    impl fmt::Debug for Ge {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Ge {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Ge
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct AndKw(SyntaxToken);
    impl fmt::Debug for AndKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for AndKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::AndKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct OrKw(SyntaxToken);
    impl fmt::Debug for OrKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for OrKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::OrKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct PlusEquals(SyntaxToken);
    impl fmt::Debug for PlusEquals {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for PlusEquals {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::PlusEquals
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct MinusEquals(SyntaxToken);
    impl fmt::Debug for MinusEquals {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for MinusEquals {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::MinusEquals
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StarEquals(SyntaxToken);
    impl fmt::Debug for StarEquals {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StarEquals {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StarEquals
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct SlashEquals(SyntaxToken);
    impl fmt::Debug for SlashEquals {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for SlashEquals {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::SlashEquals
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct PercentEquals(SyntaxToken);
    impl fmt::Debug for PercentEquals {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for PercentEquals {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::PercentEquals
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct NotKw(SyntaxToken);
    impl fmt::Debug for NotKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for NotKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::NotKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct PackageKw(SyntaxToken);
    impl fmt::Debug for PackageKw {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for PackageKw {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::PackageKw
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Dot(SyntaxToken);
    impl fmt::Debug for Dot {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Dot {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Dot
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct True(SyntaxToken);
    impl fmt::Debug for True {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for True {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::True
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct False(SyntaxToken);
    impl fmt::Debug for False {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for False {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::False
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Int(SyntaxToken);
    impl fmt::Debug for Int {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstInternedToken for Int {
        fn text_key(&self) -> TokenKey {
            self.0.text_key().unwrap()
        }
    }
    impl AstElement for Int {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Int
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Float(SyntaxToken);
    impl fmt::Debug for Float {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstInternedToken for Float {
        fn text_key(&self) -> TokenKey {
            self.0.text_key().unwrap()
        }
    }
    impl AstElement for Float {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Float
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Char(SyntaxToken);
    impl fmt::Debug for Char {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstInternedToken for Char {
        fn text_key(&self) -> TokenKey {
            self.0.text_key().unwrap()
        }
    }
    impl AstElement for Char {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Char
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct String(SyntaxToken);
    impl fmt::Debug for String {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstInternedToken for String {
        fn text_key(&self) -> TokenKey {
            self.0.text_key().unwrap()
        }
    }
    impl AstElement for String {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::String
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let tok = elem.into_token()?;
            Self::can_cast(tok.kind()).then(|| Self(tok))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
}
pub mod ast {
    use std::fmt;
    use crate::syntax::{token::*, *};
    fn children<'a, T: 'a + AstElement>(
        node: &'a SyntaxNode,
    ) -> impl Iterator<Item = T> + 'a {
        node.children_with_tokens()
            .map(|elem| match elem {
                SyntaxElementRef::Node(node) => SyntaxElement::Node(node.clone()),
                SyntaxElementRef::Token(token) => SyntaxElement::Token(token.clone()),
            })
            .filter_map(T::cast)
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct File(SyntaxNode);
    impl fmt::Debug for File {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for File {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::File
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl File {
        pub fn items(&self) -> impl Iterator<Item = Item> + '_ {
            children(&self.0)
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum Item {
        Import(Import),
        TypeDef(TypeDef),
        Function(Function),
        Constant(Constant),
    }
    impl fmt::Debug for Item {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Import(x) => fmt::Debug::fmt(x, f),
                Self::TypeDef(x) => fmt::Debug::fmt(x, f),
                Self::Function(x) => fmt::Debug::fmt(x, f),
                Self::Constant(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for Item {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(
                kind, | SyntaxKind::Import | SyntaxKind::TypeDef | SyntaxKind::Function |
                SyntaxKind::Constant
            )
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::Import => AstElement::cast(elem.clone()).map(Self::Import),
                SyntaxKind::TypeDef => AstElement::cast(elem.clone()).map(Self::TypeDef),
                SyntaxKind::Function => {
                    AstElement::cast(elem.clone()).map(Self::Function)
                }
                SyntaxKind::Constant => {
                    AstElement::cast(elem.clone()).map(Self::Constant)
                }
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Import(x) => x.range(),
                Self::TypeDef(x) => x.range(),
                Self::Function(x) => x.range(),
                Self::Constant(x) => x.range(),
            }
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Import(SyntaxNode);
    impl fmt::Debug for Import {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Import {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Import
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl Import {
        pub fn path(&self) -> Option<Path> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct TypeDef(SyntaxNode);
    impl fmt::Debug for TypeDef {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for TypeDef {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::TypeDef
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl TypeDef {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
        pub fn adt_fields(&self) -> impl Iterator<Item = AdtField> + '_ {
            children(&self.0)
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Function(SyntaxNode);
    impl fmt::Debug for Function {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Function {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Function
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl Function {
        pub fn signature(&self) -> Option<Signature> {
            children(&self.0).next()
        }
        pub fn block(&self) -> Option<Block> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Constant(SyntaxNode);
    impl fmt::Debug for Constant {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Constant {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Constant
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl Constant {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn type_ascription(&self) -> Option<TypeAscription> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Path(SyntaxNode);
    impl fmt::Debug for Path {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Path {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Path
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl Path {
        pub fn absolute(&self) -> Option<PackageKw> {
            children(&self.0).next()
        }
        pub fn path_components(&self) -> impl Iterator<Item = PathComponent> + '_ {
            children(&self.0)
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct AdtField(SyntaxNode);
    impl fmt::Debug for AdtField {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for AdtField {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::AdtField
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl AdtField {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
        pub fn type_expr(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum TypeExpr {
        TypeFunction(TypeFunction),
        Path(Path),
    }
    impl fmt::Debug for TypeExpr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::TypeFunction(x) => fmt::Debug::fmt(x, f),
                Self::Path(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for TypeExpr {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(kind, | SyntaxKind::TypeFunction | SyntaxKind::Path)
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::TypeFunction => {
                    AstElement::cast(elem.clone()).map(Self::TypeFunction)
                }
                SyntaxKind::Path => AstElement::cast(elem.clone()).map(Self::Path),
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::TypeFunction(x) => x.range(),
                Self::Path(x) => x.range(),
            }
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct TypeFunction(SyntaxNode);
    impl fmt::Debug for TypeFunction {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for TypeFunction {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::TypeFunction
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl TypeFunction {
        pub fn param_ty(&self) -> impl Iterator<Item = TypeList> + '_ {
            children(&self.0)
        }
        pub fn return_ty(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct TypeList(SyntaxNode);
    impl fmt::Debug for TypeList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for TypeList {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::TypeList
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl TypeList {
        pub fn type_expr(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct TypeAscription(SyntaxNode);
    impl fmt::Debug for TypeAscription {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for TypeAscription {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::TypeAscription
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl TypeAscription {
        pub fn type_expr(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Signature(SyntaxNode);
    impl fmt::Debug for Signature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Signature {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Signature
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl Signature {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
        pub fn param_lists(&self) -> impl Iterator<Item = ParamList> + '_ {
            children(&self.0)
        }
        pub fn return_ty(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct Block(SyntaxNode);
    impl fmt::Debug for Block {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for Block {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::Block
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl Block {
        pub fn stmts(&self) -> impl Iterator<Item = Stmt> + '_ {
            children(&self.0)
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ParamList(SyntaxNode);
    impl fmt::Debug for ParamList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ParamList {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ParamList
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ParamList {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
        pub fn type_expr(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum Expr {
        Block(Block),
        ExprIf(ExprIf),
        ExprParen(ExprParen),
        ExprPrefix(ExprPrefix),
        ExprInfix(ExprInfix),
        ExprAssign(ExprAssign),
        ExprCall(ExprCall),
        ExprClosure(ExprClosure),
        Atom(Atom),
        Path(Path),
    }
    impl fmt::Debug for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Block(x) => fmt::Debug::fmt(x, f),
                Self::ExprIf(x) => fmt::Debug::fmt(x, f),
                Self::ExprParen(x) => fmt::Debug::fmt(x, f),
                Self::ExprPrefix(x) => fmt::Debug::fmt(x, f),
                Self::ExprInfix(x) => fmt::Debug::fmt(x, f),
                Self::ExprAssign(x) => fmt::Debug::fmt(x, f),
                Self::ExprCall(x) => fmt::Debug::fmt(x, f),
                Self::ExprClosure(x) => fmt::Debug::fmt(x, f),
                Self::Atom(x) => fmt::Debug::fmt(x, f),
                Self::Path(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for Expr {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(
                kind, | SyntaxKind::Block | SyntaxKind::ExprIf | SyntaxKind::ExprParen |
                SyntaxKind::ExprPrefix | SyntaxKind::ExprInfix | SyntaxKind::ExprAssign |
                SyntaxKind::ExprCall | SyntaxKind::ExprClosure | SyntaxKind::Path
            ) || Atom::can_cast(kind)
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::Block => AstElement::cast(elem.clone()).map(Self::Block),
                SyntaxKind::ExprIf => AstElement::cast(elem.clone()).map(Self::ExprIf),
                SyntaxKind::ExprParen => {
                    AstElement::cast(elem.clone()).map(Self::ExprParen)
                }
                SyntaxKind::ExprPrefix => {
                    AstElement::cast(elem.clone()).map(Self::ExprPrefix)
                }
                SyntaxKind::ExprInfix => {
                    AstElement::cast(elem.clone()).map(Self::ExprInfix)
                }
                SyntaxKind::ExprAssign => {
                    AstElement::cast(elem.clone()).map(Self::ExprAssign)
                }
                SyntaxKind::ExprCall => {
                    AstElement::cast(elem.clone()).map(Self::ExprCall)
                }
                SyntaxKind::ExprClosure => {
                    AstElement::cast(elem.clone()).map(Self::ExprClosure)
                }
                SyntaxKind::Path => AstElement::cast(elem.clone()).map(Self::Path),
                _ => None,
            }
                .or_else(|| AstElement::cast(elem.clone()).map(Self::Atom))
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Block(x) => x.range(),
                Self::ExprIf(x) => x.range(),
                Self::ExprParen(x) => x.range(),
                Self::ExprPrefix(x) => x.range(),
                Self::ExprInfix(x) => x.range(),
                Self::ExprAssign(x) => x.range(),
                Self::ExprCall(x) => x.range(),
                Self::ExprClosure(x) => x.range(),
                Self::Atom(x) => x.range(),
                Self::Path(x) => x.range(),
            }
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum Stmt {
        Semicolon(Semicolon),
        StmtExpr(StmtExpr),
        StmtLet(StmtLet),
        StmtLoop(StmtLoop),
        StmtWhile(StmtWhile),
        StmtBreak(StmtBreak),
        StmtContinue(StmtContinue),
        StmtReturn(StmtReturn),
        Item(Item),
    }
    impl fmt::Debug for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Semicolon(x) => fmt::Debug::fmt(x, f),
                Self::StmtExpr(x) => fmt::Debug::fmt(x, f),
                Self::StmtLet(x) => fmt::Debug::fmt(x, f),
                Self::StmtLoop(x) => fmt::Debug::fmt(x, f),
                Self::StmtWhile(x) => fmt::Debug::fmt(x, f),
                Self::StmtBreak(x) => fmt::Debug::fmt(x, f),
                Self::StmtContinue(x) => fmt::Debug::fmt(x, f),
                Self::StmtReturn(x) => fmt::Debug::fmt(x, f),
                Self::Item(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for Stmt {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(
                kind, | SyntaxKind::Semicolon | SyntaxKind::StmtExpr |
                SyntaxKind::StmtLet | SyntaxKind::StmtLoop | SyntaxKind::StmtWhile |
                SyntaxKind::StmtBreak | SyntaxKind::StmtContinue | SyntaxKind::StmtReturn
            ) || Item::can_cast(kind)
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::StmtExpr => {
                    AstElement::cast(elem.clone()).map(Self::StmtExpr)
                }
                SyntaxKind::StmtLet => AstElement::cast(elem.clone()).map(Self::StmtLet),
                SyntaxKind::StmtLoop => {
                    AstElement::cast(elem.clone()).map(Self::StmtLoop)
                }
                SyntaxKind::StmtWhile => {
                    AstElement::cast(elem.clone()).map(Self::StmtWhile)
                }
                SyntaxKind::StmtBreak => {
                    AstElement::cast(elem.clone()).map(Self::StmtBreak)
                }
                SyntaxKind::StmtContinue => {
                    AstElement::cast(elem.clone()).map(Self::StmtContinue)
                }
                SyntaxKind::StmtReturn => {
                    AstElement::cast(elem.clone()).map(Self::StmtReturn)
                }
                SyntaxKind::Semicolon => {
                    AstElement::cast(elem.clone()).map(Self::Semicolon)
                }
                _ => None,
            }
                .or_else(|| AstElement::cast(elem.clone()).map(Self::Item))
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Semicolon(x) => x.range(),
                Self::StmtExpr(x) => x.range(),
                Self::StmtLet(x) => x.range(),
                Self::StmtLoop(x) => x.range(),
                Self::StmtWhile(x) => x.range(),
                Self::StmtBreak(x) => x.range(),
                Self::StmtContinue(x) => x.range(),
                Self::StmtReturn(x) => x.range(),
                Self::Item(x) => x.range(),
            }
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtExpr(SyntaxNode);
    impl fmt::Debug for StmtExpr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtExpr {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtExpr
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtExpr {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtLet(SyntaxNode);
    impl fmt::Debug for StmtLet {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtLet {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtLet
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtLet {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn type_ascription(&self) -> Option<TypeAscription> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtLoop(SyntaxNode);
    impl fmt::Debug for StmtLoop {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtLoop {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtLoop
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtLoop {
        pub fn block(&self) -> Option<Block> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtWhile(SyntaxNode);
    impl fmt::Debug for StmtWhile {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtWhile {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtWhile
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtWhile {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn block(&self) -> Option<Block> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtBreak(SyntaxNode);
    impl fmt::Debug for StmtBreak {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtBreak {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtBreak
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtBreak {}
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtContinue(SyntaxNode);
    impl fmt::Debug for StmtContinue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtContinue {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtContinue
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtContinue {}
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct StmtReturn(SyntaxNode);
    impl fmt::Debug for StmtReturn {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for StmtReturn {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::StmtReturn
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl StmtReturn {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprIf(SyntaxNode);
    impl fmt::Debug for ExprIf {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprIf {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprIf
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprIf {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn then_branch(&self) -> Option<Block> {
            children(&self.0).next()
        }
        pub fn else_branch(&self) -> Option<ExprElse> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprParen(SyntaxNode);
    impl fmt::Debug for ExprParen {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprParen {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprParen
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprParen {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprPrefix(SyntaxNode);
    impl fmt::Debug for ExprPrefix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprPrefix {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprPrefix
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprPrefix {
        pub fn prefix_op(&self) -> Option<PrefixOp> {
            children(&self.0).next()
        }
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprInfix(SyntaxNode);
    impl fmt::Debug for ExprInfix {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprInfix {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprInfix
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprInfix {
        pub fn lhs(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn infix_op(&self) -> Option<InfixOp> {
            children(&self.0).next()
        }
        pub fn rhs(&self) -> Option<Expr> {
            children(&self.0).nth(1usize)
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprAssign(SyntaxNode);
    impl fmt::Debug for ExprAssign {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprAssign {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprAssign
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprAssign {
        pub fn lhs(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn assign_op(&self) -> Option<AssignOp> {
            children(&self.0).next()
        }
        pub fn rhs(&self) -> Option<Expr> {
            children(&self.0).nth(1usize)
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprCall(SyntaxNode);
    impl fmt::Debug for ExprCall {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprCall {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprCall
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprCall {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
        pub fn args(&self) -> impl Iterator<Item = ExprList> + '_ {
            children(&self.0)
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprClosure(SyntaxNode);
    impl fmt::Debug for ExprClosure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprClosure {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprClosure
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprClosure {
        pub fn param_lists(&self) -> impl Iterator<Item = ParamList> + '_ {
            children(&self.0)
        }
        pub fn return_ty(&self) -> Option<TypeExpr> {
            children(&self.0).next()
        }
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum Atom {
        True(True),
        False(False),
        Int(Int),
        Float(Float),
        Char(Char),
        String(String),
    }
    impl fmt::Debug for Atom {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::True(x) => fmt::Debug::fmt(x, f),
                Self::False(x) => fmt::Debug::fmt(x, f),
                Self::Int(x) => fmt::Debug::fmt(x, f),
                Self::Float(x) => fmt::Debug::fmt(x, f),
                Self::Char(x) => fmt::Debug::fmt(x, f),
                Self::String(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for Atom {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(
                kind, | SyntaxKind::True | SyntaxKind::False | SyntaxKind::Int |
                SyntaxKind::Float | SyntaxKind::Char | SyntaxKind::String
            )
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::True => AstElement::cast(elem.clone()).map(Self::True),
                SyntaxKind::False => AstElement::cast(elem.clone()).map(Self::False),
                SyntaxKind::Int => AstElement::cast(elem.clone()).map(Self::Int),
                SyntaxKind::Float => AstElement::cast(elem.clone()).map(Self::Float),
                SyntaxKind::Char => AstElement::cast(elem.clone()).map(Self::Char),
                SyntaxKind::String => AstElement::cast(elem.clone()).map(Self::String),
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::True(x) => x.range(),
                Self::False(x) => x.range(),
                Self::Int(x) => x.range(),
                Self::Float(x) => x.range(),
                Self::Char(x) => x.range(),
                Self::String(x) => x.range(),
            }
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum ExprElse {
        Block(Block),
        ExprIf(ExprIf),
    }
    impl fmt::Debug for ExprElse {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Block(x) => fmt::Debug::fmt(x, f),
                Self::ExprIf(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for ExprElse {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(kind, | SyntaxKind::Block | SyntaxKind::ExprIf)
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::Block => AstElement::cast(elem.clone()).map(Self::Block),
                SyntaxKind::ExprIf => AstElement::cast(elem.clone()).map(Self::ExprIf),
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Block(x) => x.range(),
                Self::ExprIf(x) => x.range(),
            }
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum PrefixOp {
        Minus(Minus),
        NotKw(NotKw),
    }
    impl fmt::Debug for PrefixOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Minus(x) => fmt::Debug::fmt(x, f),
                Self::NotKw(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for PrefixOp {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(kind, | SyntaxKind::Minus | SyntaxKind::NotKw)
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::Minus => AstElement::cast(elem.clone()).map(Self::Minus),
                SyntaxKind::NotKw => AstElement::cast(elem.clone()).map(Self::NotKw),
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Minus(x) => x.range(),
                Self::NotKw(x) => x.range(),
            }
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum InfixOp {
        Plus(Plus),
        Minus(Minus),
        Star(Star),
        Slash(Slash),
        Percent(Percent),
        Eq(Eq),
        Ne(Ne),
        Lt(Lt),
        Le(Le),
        Gt(Gt),
        Ge(Ge),
        AndKw(AndKw),
        OrKw(OrKw),
    }
    impl fmt::Debug for InfixOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Plus(x) => fmt::Debug::fmt(x, f),
                Self::Minus(x) => fmt::Debug::fmt(x, f),
                Self::Star(x) => fmt::Debug::fmt(x, f),
                Self::Slash(x) => fmt::Debug::fmt(x, f),
                Self::Percent(x) => fmt::Debug::fmt(x, f),
                Self::Eq(x) => fmt::Debug::fmt(x, f),
                Self::Ne(x) => fmt::Debug::fmt(x, f),
                Self::Lt(x) => fmt::Debug::fmt(x, f),
                Self::Le(x) => fmt::Debug::fmt(x, f),
                Self::Gt(x) => fmt::Debug::fmt(x, f),
                Self::Ge(x) => fmt::Debug::fmt(x, f),
                Self::AndKw(x) => fmt::Debug::fmt(x, f),
                Self::OrKw(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for InfixOp {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(
                kind, | SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Star |
                SyntaxKind::Slash | SyntaxKind::Percent | SyntaxKind::Eq | SyntaxKind::Ne
                | SyntaxKind::Lt | SyntaxKind::Le | SyntaxKind::Gt | SyntaxKind::Ge |
                SyntaxKind::AndKw | SyntaxKind::OrKw
            )
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::Plus => AstElement::cast(elem.clone()).map(Self::Plus),
                SyntaxKind::Minus => AstElement::cast(elem.clone()).map(Self::Minus),
                SyntaxKind::Star => AstElement::cast(elem.clone()).map(Self::Star),
                SyntaxKind::Slash => AstElement::cast(elem.clone()).map(Self::Slash),
                SyntaxKind::Percent => AstElement::cast(elem.clone()).map(Self::Percent),
                SyntaxKind::Eq => AstElement::cast(elem.clone()).map(Self::Eq),
                SyntaxKind::Ne => AstElement::cast(elem.clone()).map(Self::Ne),
                SyntaxKind::Lt => AstElement::cast(elem.clone()).map(Self::Lt),
                SyntaxKind::Le => AstElement::cast(elem.clone()).map(Self::Le),
                SyntaxKind::Gt => AstElement::cast(elem.clone()).map(Self::Gt),
                SyntaxKind::Ge => AstElement::cast(elem.clone()).map(Self::Ge),
                SyntaxKind::AndKw => AstElement::cast(elem.clone()).map(Self::AndKw),
                SyntaxKind::OrKw => AstElement::cast(elem.clone()).map(Self::OrKw),
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Plus(x) => x.range(),
                Self::Minus(x) => x.range(),
                Self::Star(x) => x.range(),
                Self::Slash(x) => x.range(),
                Self::Percent(x) => x.range(),
                Self::Eq(x) => x.range(),
                Self::Ne(x) => x.range(),
                Self::Lt(x) => x.range(),
                Self::Le(x) => x.range(),
                Self::Gt(x) => x.range(),
                Self::Ge(x) => x.range(),
                Self::AndKw(x) => x.range(),
                Self::OrKw(x) => x.range(),
            }
        }
    }
    #[derive(Clone, Hash, PartialEq, Eq)]
    pub enum AssignOp {
        Equals(Equals),
        PlusEquals(PlusEquals),
        MinusEquals(MinusEquals),
        StarEquals(StarEquals),
        SlashEquals(SlashEquals),
        PercentEquals(PercentEquals),
    }
    impl fmt::Debug for AssignOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Equals(x) => fmt::Debug::fmt(x, f),
                Self::PlusEquals(x) => fmt::Debug::fmt(x, f),
                Self::MinusEquals(x) => fmt::Debug::fmt(x, f),
                Self::StarEquals(x) => fmt::Debug::fmt(x, f),
                Self::SlashEquals(x) => fmt::Debug::fmt(x, f),
                Self::PercentEquals(x) => fmt::Debug::fmt(x, f),
            }
        }
    }
    impl AstElement for AssignOp {
        fn can_cast(kind: SyntaxKind) -> bool {
            matches!(
                kind, | SyntaxKind::Equals | SyntaxKind::PlusEquals |
                SyntaxKind::MinusEquals | SyntaxKind::StarEquals |
                SyntaxKind::SlashEquals | SyntaxKind::PercentEquals
            )
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            match elem.kind() {
                SyntaxKind::Equals => AstElement::cast(elem.clone()).map(Self::Equals),
                SyntaxKind::PlusEquals => {
                    AstElement::cast(elem.clone()).map(Self::PlusEquals)
                }
                SyntaxKind::MinusEquals => {
                    AstElement::cast(elem.clone()).map(Self::MinusEquals)
                }
                SyntaxKind::StarEquals => {
                    AstElement::cast(elem.clone()).map(Self::StarEquals)
                }
                SyntaxKind::SlashEquals => {
                    AstElement::cast(elem.clone()).map(Self::SlashEquals)
                }
                SyntaxKind::PercentEquals => {
                    AstElement::cast(elem.clone()).map(Self::PercentEquals)
                }
                _ => None,
            }
        }
        fn range(&self) -> TextRange {
            match self {
                Self::Equals(x) => x.range(),
                Self::PlusEquals(x) => x.range(),
                Self::MinusEquals(x) => x.range(),
                Self::StarEquals(x) => x.range(),
                Self::SlashEquals(x) => x.range(),
                Self::PercentEquals(x) => x.range(),
            }
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct ExprList(SyntaxNode);
    impl fmt::Debug for ExprList {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for ExprList {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::ExprList
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl ExprList {
        pub fn expr(&self) -> Option<Expr> {
            children(&self.0).next()
        }
    }
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct PathComponent(SyntaxNode);
    impl fmt::Debug for PathComponent {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }
    impl AstElement for PathComponent {
        fn can_cast(kind: SyntaxKind) -> bool {
            kind == SyntaxKind::PathComponent
        }
        fn cast(elem: SyntaxElement) -> Option<Self> {
            let node = elem.into_node()?;
            Self::can_cast(node.kind()).then(|| Self(node))
        }
        fn range(&self) -> TextRange {
            self.0.text_range()
        }
    }
    impl PathComponent {
        pub fn name(&self) -> Option<Ident> {
            children(&self.0).next()
        }
    }
}
