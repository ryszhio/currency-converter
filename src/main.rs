use currency_converter::*;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!(
            "Mismatch arguement. Usage: <valid_currency> <to> <valid_currency>\n
            \r\rExample: cargo run -- usd npr 50.0"
        );
        std::process::exit(1);
    }
    
    let from = args[1].as_str();
    let to = args[2].as_str();
    let amount: f64 = args[3].parse().unwrap();
    
    match convert_currency(from, to, amount).await {
        Ok(converted_amount) => println!(
            "Conversion of Currency:\n{}: {:.2}\n{}: {:.2}",
            from.to_uppercase(), amount, to.to_uppercase(), converted_amount
        ),
        Err(err) => {
            println!("{err}");
            std::process::exit(1);
        }
    }
}