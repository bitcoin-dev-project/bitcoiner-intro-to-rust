
fn main() {
    match transaction_decoder_21::run(transaction_decoder_21::get_arg()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
