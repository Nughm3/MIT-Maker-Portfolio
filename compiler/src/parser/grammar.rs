use SyntaxKind::*;

use super::*;

// Inline grammar tests
// - use "//@" to specify the rule to test
// - use "//#" to create a new test
// - use "//-" to continue the last test onto the next line

const ITEM_FIRST: &[SyntaxKind] = &[ImportKw, TypeKw, FnKw, ConstKw];

const STMT_FIRST: &[SyntaxKind] = &[
    Semicolon, LetKw, LoopKw, WhileKw, BreakKw, ContinueKw, ReturnKw,
];

const EXPR_FIRST: &[SyntaxKind] = &[
    LeftBrace, // Block
    IfKw,      // ExprIf
    LeftParen, // ExprParen
    Minus,     // ExprPrefix
    NotKw,     // ExprPrefix
    FnKw,      // ExprClosure
    True,      // 'true'
    False,     // 'false'
    Int,       // 'int'
    Float,     // 'float'
    Char,      // 'char'
    String,    // 'string'
    Ident,     // Path
];

//@ file
//# fn f() {}
//- const x = 10 : Int;
pub fn file(p: &mut Parser<'_, '_, '_, impl Interner>) {
    p.start_node(File);

    while !p.eof() {
        if p.at_any(ITEM_FIRST) {
            item(p);
        } else {
            p.next_error(ITEM_FIRST);
            p.recover_to(ITEM_FIRST);
        }
    }

    p.finish_node();
}

pub fn item(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert!(p.at_any(ITEM_FIRST));
    match p.peek() {
        ImportKw => import(p),
        TypeKw => type_def(p),
        FnKw => function(p),
        ConstKw => constant(p),
        _ => p.next_error(ITEM_FIRST),
    }
}

fn nested_item(p: &mut Parser<'_, '_, '_, impl Interner>) {
    if p.at_any(&[FnKw, ConstKw]) {
        match p.peek() {
            FnKw => function(p),
            ConstKw => constant(p),
            _ => unreachable!(),
        }
    } else {
        p.start_node(ErrorTree);
        p.next_error(&[FnKw, ConstKw]);
        p.finish_node();
    }
}

//@ import
//# import foo;
//# import foo.bar;
pub fn import(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), ImportKw);
    p.start_node(Import);
    p.next();
    path(p);
    p.expect(Semicolon);
    p.finish_node();
}

//@ type_def
//# type Foo(bar: Bar, baz: Baz)
pub fn type_def(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), TypeKw);
    p.start_node(TypeDef);
    p.next();
    p.expect(Ident);

    p.expect(LeftParen);
    while !p.at(RightParen) && !p.eof() {
        p.start_node(AdtField);
        p.expect(Ident);
        p.expect(Colon);
        type_expr(p);
        if p.at(RightParen) {
            p.consume(Comma);
        } else {
            p.expect(Comma);
        }
        p.finish_node();
    }
    p.expect(RightParen);
    p.finish_node();
}

//@ type_expr
//# fn()
//# fn(Int, Int) -> Int
//# Int
pub fn type_expr(p: &mut Parser<'_, '_, '_, impl Interner>) {
    match p.peek() {
        FnKw => {
            p.start_node(TypeFunction);
            p.next();
            p.expect(LeftParen);
            while !p.at(RightParen) && !p.eof() {
                type_list(p);
            }
            p.expect(RightParen);
            if p.consume(Arrow) {
                type_expr(p);
            }
            p.finish_node();
        }
        Ident => path(p),
        _ => p.next_error(&[FnKw, Ident]),
    }
}

fn type_list(p: &mut Parser<'_, '_, '_, impl Interner>) {
    p.start_node(TypeList);
    type_expr(p);
    if p.at(RightParen) {
        p.consume(Comma);
    } else {
        p.expect(Comma);
    }
    p.finish_node();
}

//@ function
//# fn f() -> Int { return 10; }
pub fn function(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), FnKw);
    p.start_node(Function);
    signature(p);
    if p.at(LeftBrace) {
        block(p);
    }
    p.finish_node();
}

//@ signature
//# fn f(x: Int) -> Int
pub fn signature(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), FnKw);
    p.start_node(Signature);
    p.next();
    p.expect(Ident);
    p.expect(LeftParen);
    while !p.at(RightParen) && !p.eof() {
        param_list(p);
    }
    p.expect(RightParen);
    if p.consume(Arrow) {
        type_expr(p);
    }
    p.finish_node();
}

fn param_list(p: &mut Parser<'_, '_, '_, impl Interner>) {
    p.start_node(ParamList);
    p.expect(Ident);
    if p.consume(Colon) {
        type_expr(p);
    }
    if p.at(RightParen) {
        p.consume(Comma);
    } else {
        p.expect(Comma);
    }
    p.finish_node();
}

//@ constant
//# const x = 5;
//# const x = 5 : Int;
pub fn constant(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), ConstKw);
    p.start_node(Constant);
    p.next();
    p.expect(Ident);
    p.expect(Equals);
    if p.at_any(EXPR_FIRST) {
        expr(p);
    }
    if p.at(Colon) {
        p.start_node(TypeAscription);
        p.next();
        type_expr(p);
        p.finish_node();
    }
    p.expect(Semicolon);
    p.finish_node();
}

//@ block
//# {}
//# { a; b; c; }
pub fn block(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), LeftBrace);
    p.start_node(Block);
    p.expect(LeftBrace);
    while !p.at(RightBrace) && !p.eof() {
        let expected = &[STMT_FIRST, EXPR_FIRST, ITEM_FIRST].concat();
        if p.at_any(expected) {
            stmt(p);
        } else {
            p.next_error(expected);
        }
    }
    p.expect(RightBrace);
    p.finish_node();
}

//@ stmt
//# ;
//# a = 5;
//# break;
//# continue;
//# return;
//# return x;
//# fn f() {}
//# const x = 5;
pub fn stmt(p: &mut Parser<'_, '_, '_, impl Interner>) {
    match p.peek() {
        LetKw => stmt_let(p),
        LoopKw => stmt_loop(p),
        WhileKw => stmt_while(p),
        BreakKw => {
            p.start_node(StmtBreak);
            p.next();
            p.expect(Semicolon);
            p.finish_node();
        }
        ContinueKw => {
            p.start_node(StmtContinue);
            p.next();
            p.expect(Semicolon);
            p.finish_node();
        }
        ReturnKw => {
            p.start_node(StmtReturn);
            p.next();
            if !p.at(Semicolon) {
                expr(p);
            }
            p.expect(Semicolon);
            p.finish_node();
        }
        Semicolon => {
            p.next();
        }
        FnKw | ConstKw => nested_item(p),
        e if EXPR_FIRST.contains(&e) => {
            p.start_node(StmtExpr);
            if let ExprKind::Inline = expr(p) {
                p.expect(Semicolon);
            }
            p.finish_node();
        }
        // NOTE: FnKw is in EXPR_START due to ExprClosure
        _ => p.next_error(&[STMT_FIRST, EXPR_FIRST, &[ConstKw]].concat()),
    }
}

//@ stmt_let
//# let x = 5;
//# let x = 5 : Int;
pub fn stmt_let(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), LetKw);
    p.start_node(StmtLet);
    p.next();
    p.expect(Ident);
    p.expect(Equals);
    if p.at_any(EXPR_FIRST) {
        expr(p);
    }
    if p.at(Colon) {
        p.start_node(TypeAscription);
        p.next();
        type_expr(p);
        p.finish_node();
    }
    p.expect(Semicolon);
    p.finish_node();
}

//@ stmt_loop
//# loop { continue; }
pub fn stmt_loop(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), LoopKw);
    p.start_node(StmtLoop);
    p.next();
    if p.at(LeftBrace) {
        block(p);
    }
    p.finish_node();
}

//@ stmt_while
//# while x < 10 { x += 1; }
pub fn stmt_while(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), WhileKw);
    p.start_node(StmtWhile);
    p.next();
    if p.at_any(EXPR_FIRST) {
        expr(p);
    }
    if p.at(LeftBrace) {
        block(p);
    }
    p.finish_node();
}

#[derive(Debug)]
pub enum ExprKind {
    Inline,
    Block,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct BindingPower(u8);

impl BindingPower {
    const MIN: Self = BindingPower(u8::MIN);
    const MAX: Self = BindingPower(u8::MAX);

    fn of(op: SyntaxKind) -> Option<(Self, Self)> {
        let bp = [
            ASSIGNMENT_OPS,
            &[OrKw],
            &[AndKw],
            &[Eq, Ne, Lt, Le, Gt, Ge],
            &[Plus, Minus],
            &[Star, Slash, Percent],
        ]
        .iter()
        .position(|level| level.contains(&op))
        .map(|bp| bp as u8 * 2)?;

        Some(if ASSIGNMENT_OPS.contains(&op) {
            (BindingPower(bp + 2), BindingPower(bp + 1))
        } else {
            (BindingPower(bp + 1), BindingPower(bp + 2))
        })
    }
}

//@ expr
//# (a)
//# -1
//# not false
//# x + y
//# x + y * z
//# (x + y) * z
//# f()
//# f(x, y, z)
//# f()()
//# Foo(bar=baz)
//# fn() x
//# fn(x) x
//# fn(x: Int) -> Int 2 * x
//# fn(x, y, z) { return x + y + z; }
//# true
//# false
//# 123
//# 12_3
//# 0xff
//# 0xFF
//# 0b1111_0000
//# 1.23
//# 1.
//# 1e5
//# 1.0E-2_3
//# 'x'
//# '\''
//# '\0'
//# ""
//# "hello world"
//# "this is a quote: \""
//# i_dentifier
//# path.literal
pub fn expr(p: &mut Parser<'_, '_, '_, impl Interner>) -> ExprKind {
    expr_rec(p, BindingPower::MIN)
}

fn expr_rec(p: &mut Parser<'_, '_, '_, impl Interner>, bp: BindingPower) -> ExprKind {
    let checkpoint = p.checkpoint();

    let mut kind = match p.peek() {
        LeftBrace => {
            block(p);
            ExprKind::Block
        }
        IfKw => {
            expr_if(p);
            ExprKind::Block
        }
        LeftParen => {
            p.start_node(ExprParen);
            p.next();
            expr(p);
            p.expect(RightParen);
            p.finish_node();
            ExprKind::Inline
        }
        Minus | NotKw => {
            p.start_node(ExprPrefix);
            p.next();
            expr_rec(p, BindingPower::MAX);
            p.finish_node();
            ExprKind::Inline
        }
        FnKw => {
            p.start_node(ExprClosure);
            p.next();
            p.expect(LeftParen);
            while !p.at(RightParen) && !p.eof() {
                param_list(p);
            }
            p.expect(RightParen);
            if p.consume(Arrow) {
                type_expr(p);
            }
            let kind = expr(p);
            p.finish_node();
            kind
        }
        True | False | Int | Float | Char | String => {
            p.next();
            ExprKind::Inline
        }
        Ident => {
            path(p);
            ExprKind::Inline
        }
        _ => {
            p.next_error(EXPR_FIRST);
            ExprKind::Inline
        }
    };

    loop {
        match p.peek() {
            LeftParen => {
                p.start_node_at(checkpoint, ExprCall);
                p.next();
                while !p.at(RightParen) && !p.eof() {
                    p.start_node(ExprList);
                    expr(p);
                    if p.at(RightParen) {
                        p.consume(Comma);
                    } else {
                        p.expect(Comma);
                    }
                    p.finish_node();
                }
                p.expect(RightParen);
                p.finish_node();
            }
            op => {
                if let Some((l, r)) = BindingPower::of(op) {
                    if l < bp {
                        break;
                    }

                    p.start_node_at(
                        checkpoint,
                        if ASSIGNMENT_OPS.contains(&op) {
                            ExprAssign
                        } else {
                            ExprInfix
                        },
                    );

                    p.next();
                    expr_rec(p, r);
                    p.finish_node();
                } else {
                    break;
                }
            }
        }

        kind = ExprKind::Inline;
    }

    kind
}

//@ expr_if
//# if x { y; }
//# if x { y; } else { z; }
//# if x {} else if y {}
//# if x {} else if y {} else if z {} else {}
pub fn expr_if(p: &mut Parser<'_, '_, '_, impl Interner>) {
    debug_assert_eq!(p.peek(), IfKw);
    p.start_node(ExprIf);
    p.next();
    expr(p);
    if p.at(LeftBrace) {
        block(p);
    }
    if p.at(ElseKw) {
        p.next();
        match p.peek() {
            IfKw => expr_if(p),
            LeftBrace => block(p),
            _ => p.next_error(&[IfKw, LeftBrace]),
        }
    }
    p.finish_node();
}

//@ path
//# foo
//# bar.baz
//# package
//# package.foo.bar
pub fn path(p: &mut Parser<'_, '_, '_, impl Interner>) {
    p.start_node(Path);
    if !p.consume(PackageKw) {
        p.start_node(PathComponent);
        p.expect(Ident);
        p.finish_node();
    }
    while p.at(Dot) && !p.eof() {
        p.start_node(PathComponent);
        p.next();
        p.expect(Ident);
        p.finish_node();
    }
    p.finish_node();
}
