
fn main() {
    match transaction_decoder_22::run(transaction_decoder_22::get_arg()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
