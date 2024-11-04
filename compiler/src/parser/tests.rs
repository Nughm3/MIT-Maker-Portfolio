use std::fmt::Write;

use ahash::{HashMap, HashMapExt};
use cstree::build::NodeCache;

use super::{grammar::*, ParseError, Parser, SyntaxNode};
use crate::parser::SyntaxKind;

// TODO: generate tests with build script or macro, test AST layer

const GRAMMAR: &str = include_str!("grammar.rs");

#[test]
fn parser() {
    let mut test_suite: HashMap<_, Vec<_>> = HashMap::new();
    let mut context = String::new();

    for line in GRAMMAR.lines().filter(|l| l.starts_with("//")) {
        let content = line[4..].to_string();
        match &line[2..3] {
            "@" => context = content,
            "#" => test_suite.entry(context.clone()).or_default().push(content),
            "-" => {
                let test = test_suite
                    .entry(context.clone())
                    .or_default()
                    .last_mut()
                    .expect("cannot continue in an empty test group");
                test.push('\n');
                test.push_str(&content);
            }
            _ => continue,
        }
    }

    let mut cache = NodeCache::new();
    for (group, tests) in test_suite.iter() {
        for (idx, test) in tests.iter().enumerate() {
            eprintln!("{group}#{idx}");
            let (cst, parse_errors, unused_tokens) = parse(group, test, &mut cache);

            let format_cst = |cst: SyntaxNode| {
                cst.debug(cache.interner(), true)
                    .lines()
                    .skip(1)
                    .fold(String::new(), |mut s, l| {
                        let _ = writeln!(s, "{}", &l[2..]);
                        s
                    })
            };

            let mut output = String::new();

            if !parse_errors.is_empty() || !unused_tokens.is_empty() {
                output.push_str(&parse_errors.into_iter().fold(String::new(), |mut s, e| {
                    let _ = writeln!(
                        s,
                        "ERROR: expected {:?}, found {:?} ({:?})",
                        e.expected, e.found, e.range
                    );
                    s
                }));

                output.push_str(&unused_tokens.into_iter().fold(String::new(), |mut s, t| {
                    let _ = writeln!(s, "UNUSED: {t:?}");
                    s
                }));

                output.push_str("---\n");
            }

            output.push_str(&format_cst(cst));

            insta::with_settings!({
                omit_expression => true,
                description => test,
            }, {
                insta::assert_snapshot!(format!("{group}#{idx}"), output);
            });
        }
    }
}

fn parse(
    rule: &str,
    input: &str,
    cache: &mut NodeCache<'static>,
) -> (SyntaxNode, Vec<ParseError>, Vec<SyntaxKind>) {
    let mut parser = Parser::new(input, cache);
    parser.start_node(SyntaxKind::ParserTest);

    match rule {
        "file" => file(&mut parser),
        "item" => item(&mut parser),
        "import" => import(&mut parser),
        "type_def" => type_def(&mut parser),
        "type_expr" => type_expr(&mut parser),
        "function" => function(&mut parser),
        "signature" => signature(&mut parser),
        "constant" => constant(&mut parser),
        "block" => block(&mut parser),
        "stmt" => stmt(&mut parser),
        "stmt_let" => stmt_let(&mut parser),
        "stmt_loop" => stmt_loop(&mut parser),
        "stmt_while" => stmt_while(&mut parser),
        "expr" => {
            expr(&mut parser);
        }
        "expr_if" => expr_if(&mut parser),
        "path" => path(&mut parser),
        err => unreachable!("rule {err} not added to parse()"),
    }

    parser.finish_node();
    let unused_tokens = parser.tokens[parser.pos..].to_vec();
    let (cst, parse_errors) = parser.finish();
    (cst, parse_errors, unused_tokens)
}
