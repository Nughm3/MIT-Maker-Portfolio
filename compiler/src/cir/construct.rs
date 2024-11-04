use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};
use cstree::interning::{Resolver, TokenKey};

use super::{
    nodes::*,
    resolve::Env,
    types::{TypeInfo, TypeVar},
    Cir,
};
use crate::{
    index::{GlobalId, GlobalKind, Index, Module},
    syntax::{ast, AstInternedToken},
};

#[derive(Debug)]
struct LowerContext<'a> {
    index: &'a Index,
    current_module: &'a Module,
    imports: HashMap<TokenKey, (Module, Option<GlobalId>)>,
    locals: Env<LocalId>,
}

impl Cir {
    pub fn new(index: &Index) -> Self {
        let mut cir = Cir::default();
        cir.module(&mut LowerContext {
            index,
            current_module: index.contents(),
            imports: HashMap::new(),
            locals: Env::new(),
        });
        cir
    }

    fn module(&mut self, ctx: &mut LowerContext) {
        ctx.locals.scope();

        for import in ctx.current_module.imports() {
            let name = import
                .path_components()
                .last()
                .expect("empty import path")
                .name()
                .unwrap()
                .text_key();

            let resolved = ctx
                .index
                .resolve_ast_path(import.clone(), ctx.current_module);

            if !resolved.remainder().is_empty() {
                panic!("imported path does not exist"); // FIXME
            }

            // TODO: can clone be avoided?
            ctx.imports
                .insert(name, (resolved.module().clone(), resolved.global_id()));
        }

        for global_id in ctx.current_module.globals() {
            self.globals[global_id] = Some(match ctx.index[global_id].kind().clone() {
                GlobalKind::TypeDef(type_def) => Def::Type(TypeDef {
                    fields: type_def
                        .adt_fields()
                        .map(|field| {
                            let ty = self.type_expr(ctx, field.type_expr().unwrap());
                            Field {
                                name: field.name().unwrap().text_key(),
                                ty,
                            }
                        })
                        .collect(),
                }),
                GlobalKind::Function(function) => Def::Function(self.function(ctx, function)),
                GlobalKind::Constant(constant) => Def::Constant(Binding {
                    name: constant.name().unwrap().text_key(),
                    kind: BindingKind::Constant,
                    value: self.expr(ctx, constant.expr().unwrap()),
                    ty: constant
                        .type_ascription()
                        .map(|ascription| self.type_expr(ctx, ascription.type_expr().unwrap()))
                        .unwrap_or_else(|| self.tcx.insert(TypeInfo::Unknown)),
                }),
            })
        }

        ctx.locals.unscope();
    }

    fn type_expr(&mut self, ctx: &mut LowerContext, ty: ast::TypeExpr) -> TypeVar {
        let info = match ty {
            ast::TypeExpr::TypeFunction(function) => {
                let return_ty = function
                    .return_ty()
                    .map(|ty| self.type_expr(ctx, ty))
                    .unwrap_or_else(|| self.tcx.insert(TypeInfo::Void));

                TypeInfo::Function {
                    params: function
                        .param_ty()
                        .map(|ty| self.type_expr(ctx, ty.type_expr().unwrap()))
                        .collect(),
                    return_ty,
                }
            }
            ast::TypeExpr::Path(path) => TypeInfo::TypeDef(
                ctx.index
                    .resolve_ast_path(path.clone(), ctx.current_module)
                    .global_id()
                    .or_else(|| {
                        let components: Vec<_> = path
                            .path_components()
                            .map(|c| c.name().unwrap().text_key())
                            .collect();
                        components.split_first().and_then(|(head, tail)| {
                            ctx.imports.get(head).and_then(|(module, global_id)| {
                                global_id
                                    .or_else(|| ctx.index.resolve_path(tail, module).global_id())
                            })
                        })
                    })
                    .expect("could not find type"),
            ),
        };

        self.tcx.insert(info)
    }

    fn function(&mut self, ctx: &mut LowerContext<'_>, function: ast::Function) -> Function {
        ctx.locals.restricted_scope();

        let signature = function.signature().unwrap();
        let params = signature
            .param_lists()
            .enumerate()
            .map(|(i, param)| {
                let name = param.name().unwrap().text_key();
                let local_id = self.locals.push(Some(Local::Param(i)));
                ctx.locals.bind(name, local_id).expect("already bound");
                (name, self.type_expr(ctx, param.type_expr().unwrap()))
            })
            .collect();
        let return_ty = signature
            .return_ty()
            .map(|ty| self.type_expr(ctx, ty))
            .unwrap_or_else(|| self.tcx.insert(TypeInfo::Void));
        let body = self.block(ctx, function.block().unwrap());

        let function = Function {
            params,
            return_ty,
            body,
        };

        ctx.locals.unscope();
        function
    }

    fn block(&mut self, ctx: &mut LowerContext, block: ast::Block) -> Block {
        let mut exprs = Vec::new();
        ctx.locals.scope();

        let mut late_bound = VecDeque::new();
        for stmt in block.stmts() {
            if let ast::Stmt::Item(item) = stmt {
                let name = match item {
                    ast::Item::Function(function) => {
                        function.signature().unwrap().name().unwrap().text_key()
                    }
                    ast::Item::Constant(constant) => constant.name().unwrap().text_key(),
                    _ => unreachable!(),
                };

                let id = self.locals.push(None);
                ctx.locals.bind(name, id).expect("already bound");
                late_bound.push_back(id);
            }
        }

        for stmt in block.stmts() {
            exprs.push(match stmt {
                ast::Stmt::Semicolon(_) => continue,
                ast::Stmt::StmtExpr(expr) => self.expr(ctx, expr.expr().unwrap()),
                ast::Stmt::StmtLet(stmt_let) => {
                    ctx.locals.subscope();
                    let name = stmt_let.name().unwrap().text_key();
                    let binding = Binding {
                        name,
                        kind: BindingKind::Variable,
                        value: self.expr(ctx, stmt_let.expr().unwrap()),
                        ty: stmt_let
                            .type_ascription()
                            .map(|ascription| self.type_expr(ctx, ascription.type_expr().unwrap()))
                            .unwrap_or_else(|| self.tcx.insert(TypeInfo::Unknown)),
                    };
                    let id = self.locals.push(Some(Local::Binding(binding)));
                    ctx.locals.bind(name, id).expect("already bound");
                    self.exprs.push(Expr::Variable(id))
                }
                ast::Stmt::StmtLoop(stmt_loop) => {
                    let block = self.block(ctx, stmt_loop.block().unwrap());
                    self.exprs.push(Expr::Loop(block))
                }
                ast::Stmt::StmtWhile(stmt_while) => {
                    let mut block = self.block(ctx, stmt_while.block().unwrap());
                    let condition = self.expr(ctx, stmt_while.expr().unwrap());
                    let break_expr = self.exprs.push(Expr::Break);
                    let break_decision = self.exprs.push(Expr::Decision(Decision {
                        conditions: vec![condition],
                        branches: vec![Block {
                            exprs: vec![break_expr],
                        }],
                        default: None,
                    }));
                    block.exprs.insert(0, break_decision);
                    self.exprs.push(Expr::Loop(block))
                }
                ast::Stmt::StmtBreak(_) => self.exprs.push(Expr::Break),
                ast::Stmt::StmtContinue(_) => self.exprs.push(Expr::Continue),
                ast::Stmt::StmtReturn(stmt_return) => {
                    let expr = stmt_return.expr().map(|e| self.expr(ctx, e));
                    self.exprs.push(Expr::Return(expr))
                }
                ast::Stmt::Item(item) => {
                    let local = match item {
                        ast::Item::Function(function) => {
                            Local::Function(self.function(ctx, function))
                        }
                        ast::Item::Constant(constant) => {
                            ctx.locals.subscope();
                            Local::Binding(Binding {
                                name: constant.name().unwrap().text_key(),
                                kind: BindingKind::Constant,
                                value: self.expr(ctx, constant.expr().unwrap()),
                                ty: constant
                                    .type_ascription()
                                    .map(|ascription| {
                                        self.type_expr(ctx, ascription.type_expr().unwrap())
                                    })
                                    .unwrap_or_else(|| self.tcx.insert(TypeInfo::Unknown)),
                            })
                        }
                        _ => unreachable!(),
                    };

                    let id = late_bound.pop_front().expect("not late bound");
                    assert!(self.locals[id].is_none(), "already bound");
                    self.locals[id] = Some(local);
                    continue;
                }
            })
        }

        ctx.locals.unscope();
        Block { exprs }
    }

    fn expr(&mut self, ctx: &mut LowerContext, expr: ast::Expr) -> ExprId {
        let expr = match expr {
            ast::Expr::Block(block) => Expr::Block(self.block(ctx, block)),
            ast::Expr::ExprIf(expr_if) => {
                let mut decision = Decision {
                    conditions: Vec::with_capacity(1),
                    branches: Vec::with_capacity(1),
                    default: None,
                };

                fn lower_expr_if(
                    cir: &mut Cir,
                    ctx: &mut LowerContext,
                    expr_if: ast::ExprIf,
                    decision: &mut Decision,
                ) {
                    decision
                        .conditions
                        .push(cir.expr(ctx, expr_if.expr().unwrap()));
                    decision
                        .branches
                        .push(cir.block(ctx, expr_if.then_branch().unwrap()));
                    if let Some(else_branch) = expr_if.else_branch() {
                        match else_branch {
                            ast::ExprElse::Block(block) => {
                                decision.default = Some(cir.block(ctx, block))
                            }
                            ast::ExprElse::ExprIf(expr_if) => {
                                lower_expr_if(cir, ctx, expr_if, decision)
                            }
                        }
                    }
                }

                lower_expr_if(self, ctx, expr_if, &mut decision);
                Expr::Decision(decision)
            }
            ast::Expr::ExprParen(paren) => return self.expr(ctx, paren.expr().unwrap()),
            ast::Expr::ExprPrefix(prefix) => Expr::Prefix {
                op: match prefix.prefix_op().unwrap() {
                    ast::PrefixOp::Minus(_) => PrefixOp::Neg,
                    ast::PrefixOp::NotKw(_) => PrefixOp::Not,
                },
                expr: self.expr(ctx, prefix.expr().unwrap()),
            },
            ast::Expr::ExprInfix(infix) => Expr::Infix {
                lhs: self.expr(ctx, infix.lhs().unwrap()),
                op: match infix.infix_op().unwrap() {
                    ast::InfixOp::Plus(_) => InfixOp::Add,
                    ast::InfixOp::Minus(_) => InfixOp::Sub,
                    ast::InfixOp::Star(_) => InfixOp::Mul,
                    ast::InfixOp::Slash(_) => InfixOp::Div,
                    ast::InfixOp::Percent(_) => InfixOp::Rem,
                    ast::InfixOp::Eq(_) => InfixOp::Eq,
                    ast::InfixOp::Ne(_) => InfixOp::Ne,
                    ast::InfixOp::Lt(_) => InfixOp::Lt,
                    ast::InfixOp::Le(_) => InfixOp::Le,
                    ast::InfixOp::Gt(_) => InfixOp::Gt,
                    ast::InfixOp::Ge(_) => InfixOp::Ge,
                    ast::InfixOp::AndKw(_) => InfixOp::And,
                    ast::InfixOp::OrKw(_) => InfixOp::Or,
                },
                rhs: self.expr(ctx, infix.rhs().unwrap()),
            },
            ast::Expr::ExprAssign(assign) => {
                let lhs = self.expr(ctx, assign.lhs().unwrap());
                let mut rhs = self.expr(ctx, assign.rhs().unwrap());

                let assign_op = assign.assign_op().unwrap();
                if !matches!(assign_op, ast::AssignOp::Equals(_)) {
                    let op = match assign_op {
                        ast::AssignOp::PlusEquals(_) => InfixOp::Add,
                        ast::AssignOp::MinusEquals(_) => InfixOp::Sub,
                        ast::AssignOp::StarEquals(_) => InfixOp::Mul,
                        ast::AssignOp::SlashEquals(_) => InfixOp::Div,
                        ast::AssignOp::PercentEquals(_) => InfixOp::Rem,
                        _ => unreachable!(),
                    };

                    rhs = self.exprs.push(Expr::Infix { lhs, op, rhs });
                }

                Expr::Assign { lhs, rhs }
            }
            ast::Expr::ExprCall(call) => Expr::Call {
                function: self.expr(ctx, call.expr().unwrap()),
                args: call
                    .args()
                    .map(|arg| self.expr(ctx, arg.expr().unwrap()))
                    .collect(),
            },
            ast::Expr::ExprClosure(_closure) => todo!(),
            ast::Expr::Atom(atom) => Expr::Atom(match atom {
                ast::Atom::True(_) => Atom::Bool(true),
                ast::Atom::False(_) => Atom::Bool(false),
                ast::Atom::Int(int) => {
                    let s = ctx.index.interner().resolve(int.text_key());
                    Atom::Int(s.parse().unwrap())
                }
                ast::Atom::Float(float) => {
                    let s = ctx.index.interner().resolve(float.text_key());
                    Atom::Float(s.parse().unwrap())
                }
                ast::Atom::Char(char) => {
                    let s = ctx.index.interner().resolve(char.text_key());
                    let c = match &s[0..1] {
                        "\\" if s.len() == 2 => match &s[1..2] {
                            "\\" => '\\',
                            "n" => '\n',
                            "r" => '\r',
                            "t" => '\t',
                            "0" => '\0',
                            "'" => '\'',
                            _ => unreachable!(),
                        },
                        _ if s.len() == 1 => s.parse().unwrap(),
                        _ => unreachable!(),
                    };
                    Atom::Char(c)
                }
                ast::Atom::String(string) => Atom::String(string.text_key()),
            }),
            ast::Expr::Path(path) => {
                let components: Vec<_> = path
                    .path_components()
                    .map(|c| c.name().unwrap().text_key())
                    .collect();

                if let Some(&local_id) = components.first().and_then(|&c| ctx.locals.resolve(c)) {
                    Expr::Local(local_id)
                } else {
                    let (global, remainder) = if let Some((module, global_id)) =
                        components.first().and_then(|c| ctx.imports.get(c))
                    {
                        global_id
                            .map(|global_id| (Expr::Global(global_id), Vec::new()))
                            .or_else(|| {
                                components.split_first().and_then(|(_, tail)| {
                                    let resolved = ctx.index.resolve_path(tail, module);
                                    resolved.global_id().map(|global_id| {
                                        (Expr::Global(global_id), resolved.remainder().to_vec())
                                    })
                                })
                            })
                            .expect("failed to resolve import")
                    } else {
                        let resolved = ctx.index.resolve_ast_path(path, ctx.current_module);
                        (
                            Expr::Global(resolved.global_id().expect("failed to resolve")),
                            resolved.remainder().to_vec(),
                        )
                    };

                    remainder.iter().fold(global, |acc, &name| Expr::Access {
                        expr: self.exprs.push(acc),
                        name,
                    })
                }
            }
        };

        self.exprs.push(expr)
    }
}
