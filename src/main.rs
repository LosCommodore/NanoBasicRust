pub mod tokenizer;

fn main() {
    println!("Starting program");
    let my_token = tokenizer::match_line("a = 12");
    println!("{:#?}", my_token);
}
