extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "latex-formula-grammar.pest"]
struct LaTeXFormulaParser;

#[allow(dead_code)]
const EXAMPLE_FORMULA_0: &'static str = "\\alpha";

#[allow(dead_code)]
const EXAMPLE_FORMULA_1: &'static str = "E = mc^2";

#[allow(dead_code)]
const EXAMPLE_FORMULA_2: &'static str = "e^{i \\pi} + 1 = 0"; // The Euler Identity

#[allow(dead_code)]
const EXAMPLE_FORMULA_3: &'static str = "\\sum_{i=1}^N (y_i – X_i \\cdot \\beta)^2"; // Latex formula from Word

fn main() {
    let parsed = LaTeXFormulaParser::parse(Rule::expression, EXAMPLE_FORMULA_3).unwrap();
    println!("{:#?}", parsed);
}
