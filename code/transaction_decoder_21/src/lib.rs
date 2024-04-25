mod transaction;
use self::transaction::{Decodable, Transaction,};
use std::error::Error;
use clap::{arg, value_parser, Command};

pub fn get_arg() -> String {
    let matches = Command::new("Bitcoin Transaction Decoder")
        .version("1.0")
        .about("Decodes a raw transaction")
        .arg(
            arg!([RAW_TRANSACTION])
                .value_parser(value_parser!(String))
                .required(true)
        )
        .get_matches();

    matches
        .get_one::<String>("RAW_TRANSACTION")
        .cloned()
        .expect("raw transaction is required")
}

pub fn decode(raw_transaction_hex: String) -> Result<Transaction, Box<dyn Error>> {
    let transaction_bytes = hex::decode(raw_transaction_hex).map_err(|e| format!("Hex decoding error: {}", e))?;
    let mut bytes_slice = transaction_bytes.as_slice();
    Ok(Transaction::consensus_decode(&mut bytes_slice)?)
}

pub fn run(raw_transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction = decode(raw_transaction_hex)?;
    Ok(serde_json::to_string_pretty(&transaction)?)
}
