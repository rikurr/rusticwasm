use nom::bytes::complete::tag;
use rusticwasm::wat::token::{bws, func, param, ws};

fn main() {
    // test.watを読み込む
    // let wat_program = std::fs::read_to_string("test.wat").unwrap();

    // println!("{}", wat_program);

    fn parser(input: &str) -> nom::IResult<&str, &str> {
        bws(param)(input)
    }

    let a = parser("p a r a m");

    println!("a:{:?}", a);

    let aa = func("  func $add (param $lhs i32) (param $rhs i32) (result i32)");
    println!("aa:{:?}", aa);

    // let input = /* WATプログラムの文字列 */;

    // // 字句解析
    // let mut lexer = WatLexer::new(&input);
    // let tokens = lexer.tokenize().unwrap();

    // // 構文解析
    // let mut parser = WatParser::new(tokens);
    // let ast = parser.parse().unwrap();

    // // 意味解析
    // let mut analyzer = WatAnalyzer::new();
    // analyzer.analyze(&ast).unwrap();

    // // コード生成
    // let mut compiler = WatCompiler::new();
    // let wasm_binary = compiler.compile(&ast).unwrap();
}
