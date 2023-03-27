// use rusticwasm::wat;

fn main() {
    // test.watを読み込む
    let wat_program = std::fs::read_to_string("test.wat").unwrap();

    println!("{}", wat_program);

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
