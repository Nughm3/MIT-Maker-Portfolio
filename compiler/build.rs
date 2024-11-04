#![allow(clippy::disallowed_types)]

use std::{collections::HashMap, fs};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use ungrammar::{Grammar, Node, Rule};

// TODO: split into crate

const UNGRAMMAR: &str = "narwhal.ungram";
const OUTPUT: &str = "src/syntax/generated.rs";

fn main() {
    println!("cargo:rerun-if-changed={UNGRAMMAR}");
    let input = fs::read_to_string(UNGRAMMAR).expect("failed to read file");

    let grammar: Grammar = input.parse().expect("invalid ungrammar");
    let node_types = grammar
        .iter()
        .map(|node| {
            (
                node,
                if let Rule::Alt(_) = &grammar[node].rule {
                    NodeType::Enum
                } else {
                    NodeType::Struct
                },
            )
        })
        .collect();

    let kind = generate_kind(&grammar, &node_types);
    let token = generate_tokens(&grammar);
    let ast = generate_ast(&grammar, &node_types);

    let tokens = quote! {
        pub mod kind { #kind }
        pub mod token { #token }
        pub mod ast { #ast }
    };

    let file = syn::parse2(tokens).expect("invalid token stream");
    fs::write(OUTPUT, prettyplease::unparse(&file)).expect("failed to write output");
}

fn token_map(token: &str) -> TokenData {
    fn lex(expr: &str, lex: &str) -> TokenData {
        TokenData {
            expr: expr.to_string(),
            lex: lex.to_string(),
            regex: false,
        }
    }

    fn regex(expr: &str, regex: &str) -> TokenData {
        TokenData {
            expr: expr.to_string(),
            lex: regex.to_string(),
            regex: true,
        }
    }

    match token {
        // Delimiters
        "(" => lex("LeftParen", "("),
        ")" => lex("RightParen", ")"),
        "{" => lex("LeftBrace", "{"),
        "}" => lex("RightBrace", "}"),

        // Symbols (operators)
        "+" => lex("Plus", "+"),
        "-" => lex("Minus", "-"),
        "*" => lex("Star", "*"),
        "/" => lex("Slash", "/"),
        "%" => lex("Percent", "%"),
        "==" => lex("Eq", "=="),
        "!=" => lex("Ne", "!="),
        "<" => lex("Lt", "<"),
        "<=" => lex("Le", "<="),
        ">" => lex("Gt", ">"),
        ">=" => lex("Ge", ">="),
        "not" => lex("NotKw", "not"),
        "and" => lex("AndKw", "and"),
        "or" => lex("OrKw", "or"),
        "=" => lex("Equals", "="),
        "+=" => lex("PlusEquals", "+="),
        "-=" => lex("MinusEquals", "-="),
        "*=" => lex("StarEquals", "*="),
        "/=" => lex("SlashEquals", "/="),
        "%=" => lex("PercentEquals", "%="),

        // Symbols (punctuation)
        "." => lex("Dot", "."),
        "," => lex("Comma", ","),
        ":" => lex("Colon", ":"),
        ";" => lex("Semicolon", ";"),
        "->" => lex("Arrow", "->"),

        // Keywords
        "import" => lex("ImportKw", "import"),
        "package" => lex("PackageKw", "package"),
        "type" => lex("TypeKw", "type"),
        "fn" => lex("FnKw", "fn"),
        "const" => lex("ConstKw", "const"),
        "let" => lex("LetKw", "let"),
        "loop" => lex("LoopKw", "loop"),
        "while" => lex("WhileKw", "while"),
        "break" => lex("BreakKw", "break"),
        "continue" => lex("ContinueKw", "continue"),
        "return" => lex("ReturnKw", "return"),
        "if" => lex("IfKw", "if"),
        "else" => lex("ElseKw", "else"),

        // Literals
        "true" => lex("True", "true"),
        "false" => lex("False", "false"),
        "int" => regex("Int", r"\d[\d_]*|0x[\da-fA-F][\da-fA-F_]*|0b[01][01_]*"),
        "float" => regex(
            "Float",
            r"\d[\d_]*(\.(\d[\d_]*)?|\.\d[\d_]*[eE][+-]?\d[\d_]*|[eE][+-]?\d[\d_]*)",
        ),
        "char" => regex("Char", r"'([^']|\\[\\nrt0'])'"),
        "string" => regex("String", r#""([^"]|\\[\\nrt0"])*""#),
        "ident" => regex("Ident", r"[a-zA-Z_][a-zA-Z\d_]*"),

        err => panic!("token not added to token_map(): {err}"),
    }
}

#[derive(Debug)]
enum NodeType {
    Struct,
    Enum,
}

#[derive(Debug)]
enum NodeData {
    Struct(Struct),
    Enum(Enum),
}

#[derive(Debug)]
struct Struct {
    name: String,
    fields: Vec<Field>,
    type_repeat: HashMap<String, Repeat>,
}

impl Struct {
    fn get_repeat(&mut self, ty: &str, rule: &Rule) -> usize {
        match self.type_repeat.get_mut(ty) {
            Some(Repeat::One(n)) => {
                *n += 1;
                *n
            }
            Some(Repeat::Many) => {
                panic!("rule '{rule:?}' uses type '{ty}' which was already used before",)
            }
            None => {
                self.type_repeat.insert(ty.to_string(), Repeat::One(0));
                0
            }
        }
    }

    fn check_repeat(&mut self, ty: &str, r: &Rule) {
        if self.type_repeat.contains_key(ty) {
            panic!("rule '{r:?}' uses type '{ty}' which was already used before",);
        }
    }
}

#[derive(Debug)]
struct Field {
    kind: FieldKind,
    name: String,
    ty: String,
    repeat: Repeat,
}

#[derive(Debug)]
enum FieldKind {
    Node,
    Token,
}

#[derive(Debug)]
enum Repeat {
    One(usize),
    Many,
}

#[derive(Debug)]
struct Enum {
    name: String,
    nodes: Vec<Variant>,
    tokens: Vec<String>,
}

#[derive(Debug)]
struct Variant {
    name: String,
    node: Node,
}

#[derive(Debug)]
struct TokenData {
    expr: String,
    lex: String,
    regex: bool,
}

impl TokenData {
    fn static_text(&self) -> Option<&str> {
        (!self.regex).then_some(&self.lex)
    }
}

fn generate_kind(grammar: &Grammar, node_types: &HashMap<Node, NodeType>) -> TokenStream {
    let token_data: Vec<_> = grammar
        .tokens()
        .map(|n| token_map(&grammar[n].name))
        .collect();

    let tokens: Vec<_> = token_data
        .iter()
        .map(|token| {
            let ident = format_ident!("{}", token.expr);
            let lex = token.lex.clone();
            let static_text = token
                .static_text()
                .map(|s| quote!(#[static_text(#s)]))
                .unwrap_or_default();

            if token.regex {
                quote! {
                    #[regex(#lex)]
                    #static_text
                    #ident
                }
            } else {
                quote! {
                    #[token(#lex)]
                    #static_text
                    #ident
                }
            }
        })
        .collect();

    let nodes: Vec<_> = {
        let nodes: Vec<_> = grammar
            .iter()
            .filter_map(|n| match node_types[&n] {
                NodeType::Struct => Some(grammar[n].name.to_string()),
                NodeType::Enum => None,
            })
            .collect();

        nodes.iter().map(|node| format_ident!("{node}")).collect()
    };

    quote! {
        use logos::Logos;
        use cstree::{Syntax, text::{TextRange, TextSize}};

        #[repr(u32)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Logos, Syntax)]
        pub enum SyntaxKind {
            #(#tokens,)*
            #(#nodes,)*

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
                logos::Logos::lexer(input).spanned().map(move |(token, range)| {
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
}

fn generate_tokens(grammar: &Grammar) -> TokenStream {
    let tokens = grammar.tokens().map(|token| {
        let TokenData { expr, regex, .. } = token_map(&grammar[token].name);

        let ident = format_ident!("{expr}");

        let token_trait_impl = if regex {
            quote! {
                impl AstInternedToken for #ident {
                    fn text_key(&self) -> TokenKey {
                        self.0.text_key().unwrap()
                    }
                }
            }
        } else {
            quote!()
        };

        quote! {
            #[derive(Clone, PartialEq, Eq, Hash)]
            pub struct #ident(SyntaxToken);

            impl fmt::Debug for #ident {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    fmt::Debug::fmt(&self.0, f)
                }
            }

            #token_trait_impl

            impl AstElement for #ident {
                fn can_cast(kind: SyntaxKind) -> bool {
                    kind == SyntaxKind::#ident
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
    });

    quote! {
        use std::fmt;
        use crate::syntax::*;

        #(#tokens)*
    }
}

fn generate_ast(grammar: &Grammar, node_types: &HashMap<Node, NodeType>) -> TokenStream {
    let nodes = generate_nodes(grammar, node_types);
    let nodes = nodes
			.into_iter()
			.map(|node| {
				match node {
					NodeData::Struct(s) => {
						let name = format_ident!("{}", s.name);
						let fields = s.fields.into_iter().map(|f|
							match f.kind {
                                FieldKind::Node => {
        							let name = format_ident!("{}", f.name);
        							let ty = format_ident!("{}", f.ty);

    								match f.repeat {
    									Repeat::Many => quote! {
    										pub fn #name(&self) -> impl Iterator<Item = #ty> + '_ {
    											children(&self.0)
    										}
    									},
    									Repeat::One(n) => {
                                            let call = if n == 0 {
    											quote!(children(&self.0).next())
                                            } else {
    											quote!(children(&self.0).nth(#n))
                                            };

                                            quote! {
        										pub fn #name(&self) -> Option<#ty> {
                                                    #call
        										}
        									}
                                        },
    								}
    							},
                                FieldKind::Token => {
        							let name = format_ident!("{}", f.name);
        							let ty = format_ident!("{}", f.ty);

    								match f.repeat {
    									Repeat::Many => {
    										quote! {
    											pub fn #name(&self) -> impl Iterator<Item = #ty> + '_ {
    												children(&self.0)
    											}
    										}
    									},
    									Repeat::One(n) => {
                                            let call = if n == 0 {
    											quote!(children(&self.0).next())
                                            } else {
    											quote!(children(&self.0).nth(#n))
                                            };

                                            quote! {
        										pub fn #name(&self) -> Option<#ty> {
                                                    #call
        										}
        									}
    									},
    								}
                                }
							}
						);

						quote! {
							#[derive(Clone, PartialEq, Eq, Hash)]
							pub struct #name(SyntaxNode);

							impl fmt::Debug for #name {
								fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
								    fmt::Debug::fmt(&self.0, f)
								}
							}

							impl AstElement for #name {
								fn can_cast(kind: SyntaxKind) -> bool {
									kind == SyntaxKind::#name
								}

								fn cast(elem: SyntaxElement) -> Option<Self> {
									let node = elem.into_node()?;
									Self::can_cast(node.kind()).then(|| Self(node))
								}

								fn range(&self) -> TextRange {
									self.0.text_range()
								}
							}

							impl #name {
								#(#fields)*
							}
						}
					},
					NodeData::Enum(e) => {
						let name = format_ident!("{}", e.name);
						let token_variants: Vec<_> = e.tokens.iter().map(|variant| format_ident!("{variant}")).collect();

						let node_variants: Vec<_> =
							e.nodes.iter().map(|variant| format_ident!("{}", variant.name)).collect();

						let mut struct_variants = Vec::new();
						let mut enum_variants = Vec::new();

						for variant in e.nodes {
                            let name = format_ident!("{}", variant.name);
							match node_types[&variant.node] {
								NodeType::Struct => struct_variants.push(name),
								NodeType::Enum => enum_variants.push(name),
							}
						}

						quote! {
                            #[derive(Clone, Hash, PartialEq, Eq)]
							pub enum #name {
								#(#token_variants(#token_variants),)*
								#(#node_variants(#node_variants),)*
							}

							impl fmt::Debug for #name {
								fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
									match self {
										#(Self::#token_variants(x) => fmt::Debug::fmt(x, f),)*
										#(Self::#node_variants(x) => fmt::Debug::fmt(x, f),)*
									}
								}
							}

							impl AstElement for #name {
								fn can_cast(kind: SyntaxKind) -> bool {
									matches!(kind, #(| SyntaxKind::#token_variants)* #(| SyntaxKind::#struct_variants)*)
									#(|| #enum_variants::can_cast(kind))*
								}

								fn cast(elem: SyntaxElement) -> Option<Self> {
									match elem.kind() {
										#(SyntaxKind::#struct_variants => AstElement::cast(elem.clone()).map(Self::#struct_variants),)*
										#(SyntaxKind::#token_variants => AstElement::cast(elem.clone()).map(Self::#token_variants),)*
										_ => None,
									} #(.or_else(|| AstElement::cast(elem.clone()).map(Self::#enum_variants)))*
								}

								fn range(&self) -> TextRange {
									match self {
										#(Self::#token_variants(x) => x.range(),)*
										#(Self::#node_variants(x) => x.range(),)*
									}
								}
							}
						}
					},
				}
			});

    quote! {
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

        #(#nodes)*
    }
}

fn generate_nodes(grammar: &Grammar, node_types: &HashMap<Node, NodeType>) -> Vec<NodeData> {
    grammar
        .iter()
        .map(|n| {
            let node = &grammar[n];
            match node_types[&n] {
                NodeType::Struct => {
                    let mut s = Struct {
                        name: node.name.clone(),
                        fields: Vec::new(),
                        type_repeat: HashMap::new(),
                    };

                    lower_rule(grammar, &mut s, None, &node.rule);
                    NodeData::Struct(s)
                }
                NodeType::Enum => {
                    let e = lower_enum(grammar, node.name.clone(), &node.rule);
                    NodeData::Enum(e)
                }
            }
        })
        .collect()
}

fn lower_rule(grammar: &Grammar, out: &mut Struct, label: Option<&String>, rule: &Rule) {
    if lower_comma_list(grammar, out, label, rule) {
        return;
    }

    match rule {
        Rule::Labeled { label, rule } => lower_rule(grammar, out, Some(label), rule),
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let index = out.get_repeat(&ty, rule);

            out.fields.push(Field {
                kind: FieldKind::Node,
                name: label.cloned().unwrap_or_else(|| to_snake_case(&ty)),
                ty,
                repeat: Repeat::One(index),
            });
        }
        Rule::Token(token) => {
            let token = &grammar[*token].name;
            let ty = token_map(token).expr;
            let index = out.get_repeat(&ty, rule);

            if let Some(label) = label {
                out.fields.push(Field {
                    kind: FieldKind::Token,
                    name: label.clone(),
                    ty,
                    repeat: Repeat::One(index),
                })
            }
        }
        Rule::Seq(rules) => {
            for rule in rules {
                lower_rule(grammar, out, label, rule);
            }
        }
        Rule::Alt(_) => panic!("alternation rule is not allowed here: '{rule:?}'",),
        Rule::Opt(rule) => lower_rule(grammar, out, label, rule),
        Rule::Rep(rule) => match rule.as_ref() {
            Rule::Node(node) => {
                let ty = grammar[*node].name.clone();
                out.check_repeat(&ty, rule);

                out.fields.push(Field {
                    kind: FieldKind::Node,
                    name: label
                        .cloned()
                        .unwrap_or_else(|| pluralize(&to_snake_case(&ty))),
                    ty,
                    repeat: Repeat::Many,
                });
            }
            Rule::Token(token) => {
                let token = &grammar[*token].name;
                let ty = token_map(token).expr;
                out.fields.push(Field {
                    kind: FieldKind::Token,
                    name: label
                        .cloned()
                        .unwrap_or_else(|| pluralize(&to_snake_case(&ty))),
                    ty,
                    repeat: Repeat::Many,
                })
            }
            _ => {}
        },
    }
}

fn lower_enum(grammar: &Grammar, name: String, rule: &Rule) -> Enum {
    let mut nodes = Vec::new();
    let mut tokens = Vec::new();

    let Rule::Alt(alts) = rule else {
        panic!("expected an alternation rule, got '{rule:?}'",)
    };

    for alt in alts {
        match alt {
            Rule::Node(node) => {
                let data = &grammar[*node];
                nodes.push(Variant {
                    name: data.name.clone(),
                    node: *node,
                });
            }
            Rule::Token(token) => {
                let token = token_map(&grammar[*token].name).expr;
                tokens.push(token);
            }
            _ => panic!("expected node or token, got {rule:?}",),
        }
    }

    Enum {
        name,
        nodes,
        tokens,
    }
}

// (T (',' T)* ','?)
fn lower_comma_list(grammar: &Grammar, out: &mut Struct, label: Option<&String>, r: &Rule) -> bool {
    let Rule::Seq(rule) = r else {
        return false;
    };

    let [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_comma)] = rule.as_slice() else {
        return false;
    };

    let Rule::Seq(repeat) = repeat.as_ref() else {
        return false;
    };

    match repeat.as_slice() {
        [comma, Rule::Node(n)] if comma == trailing_comma.as_ref() && n == node => {}
        _ => return false,
    }

    let ty = grammar[*node].name.clone();
    let name = label
        .cloned()
        .unwrap_or_else(|| pluralize(&to_snake_case(&ty)));

    out.check_repeat(&ty, r);
    out.fields.push(Field {
        kind: FieldKind::Node,
        name,
        ty,
        repeat: Repeat::Many,
    });

    true
}

fn to_snake_case(x: &str) -> String {
    const RUST_KEYWORDS: &[&str] = &[
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
        "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
        "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub",
        "pure", "ref", "return", "Self", "self", "sizeof", "static", "struct", "super", "trait",
        "true", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];

    let mut s = String::with_capacity(x.len());
    let mut last = '_';
    for c in x.chars() {
        if c.is_ascii_uppercase() {
            if last != '_' {
                s.push('_');
            }
            s.push(c.to_ascii_lowercase());
        } else {
            s.push(c);
        }
        last = c;
    }

    if RUST_KEYWORDS.contains(&s.as_str()) {
        s.push('_');
    }
    s
}

fn pluralize(x: &str) -> String {
    let mut s = String::with_capacity(x.len() + 1);
    s.push_str(x);
    if s.ends_with('_') {
        s.pop();
    }
    s.push('s');
    s
}
