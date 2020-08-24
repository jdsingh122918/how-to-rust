use std::io;

fn main() {
    loop {
        println!("Please type quit to exit the program.");
        println!("Enter 1 for converting celcius to farenheit.");
        println!("Enter 2 for converting farenheit to celcius");
        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line");
        let conversion_type: u32 = match conversion_type.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please enter the temperature in decimals to convert: ");
        let mut original_temperature = String::new();
        io::stdin()
            .read_line(&mut original_temperature)
            .expect("Failed to read line");
        let original_temperature: f64 = original_temperature.trim().parse::<f64>().unwrap();
        let converted_temperature: f64;

        if conversion_type == 1 {
            converted_temperature = (9.0 * original_temperature) / 5.0 + 32.0;
            println!(
                "Converted temperature in farenheit: {}",
                converted_temperature
            );
        } else if conversion_type == 2 {
            converted_temperature = (original_temperature - 32.0) / 1.8;
            println!(
                "Converted temperature in celcius: {}",
                converted_temperature
            );
        }
    }
}
