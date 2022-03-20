use santiago::grammar::grammar_builder::GrammarBuilder;
use santiago::grammar::grammar_rule::GrammarRule;
use santiago::lexer::lex;
use santiago::lexer::lexeme::Lexeme;
use santiago::lexer::lexer_builder::LexerBuilder;
use santiago::lexer::lexer_rule::LexerRule;
use santiago::parser::forest::Forest;
use santiago::parser::parse::parse;

fn main() -> Result<(), String> {
    // Let's define an ambiguous grammar for adding integer numbers:
    //
    //   sum = sum plus sum
    //       | INT
    //
    //   plus = +
    //
    let grammar: Vec<GrammarRule> = GrammarBuilder::new()
        .map_to_rules("sum", &["sum", "plus", "sum"])
        .map_to_lexemes("sum", &["INT"])
        .map_to_lexemes("plus", &["+"])
        .finish();

    // A lexer splits the input string into units
    // of related characters called "Lexemes"
    //
    // For this calculator we are interested in the "+" operator
    // and the digits of the integer numbers:
    //
    //   +    := "+" (a character)
    //   INT  := \d+ (regular expression for 1 or more digits)
    //   WS   := " " (ignore whitespace)
    //
    let lexing_rules: Vec<LexerRule> = LexerBuilder::new()
        .string(&["INITIAL"], "+", "+", |lexer| lexer.take())
        .pattern(&["INITIAL"], "INT", r"\d+", |lexer| lexer.take())
        .string(&["INITIAL"], "WS", " ", |lexer| lexer.skip())
        .finish();

    // Let's start by tokenizing the input
    let input = "1 + 2 + 3";
    let lexemes: Vec<Lexeme> = lex(&lexing_rules, input);

    // Now parse!
    let abstract_syntax_trees: Vec<Forest> = parse(&grammar, &lexemes)?;

    // And print the results:
    println!("input: {:?}", input);

    println!("lexemes:");
    for lexeme in &lexemes {
        println!("  {lexeme:?}");
    }

    println!("Grammar:");
    for rule in &grammar {
        println!("  {rule}");
    }

    println!("AST:");
    for ast in abstract_syntax_trees {
        println!("{ast}");
    }

    Ok(())
}
