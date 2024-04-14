fn main() {
    match transaction_decoder_19::run(transaction_decoder_19::get_arg()) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{}", e),
    }
}
