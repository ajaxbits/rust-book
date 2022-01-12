fn main() {
    /// Converts Ferenheight to Celsius
    enum TemperatureType {
        Celcius,
        Fahrenheit,
    }
    fn f_to_c(fahrenheit: i32) -> i32 {
        ((fahrenheit - 32) * 5) / 9
    }
    fn c_to_f(celcius: i32) -> i32 {
        ((celcius * 9) / 5) + 32
    }

    fn temperature_converter(temperature: i32, temperature_type: TemperatureType) -> i32 {
        match temperature_type {
            TemperatureType::Celcius => c_to_f(temperature),
            TemperatureType::Fahrenheit => f_to_c(temperature),
        }
    }

    // Generate the nth Fibonacci number
    fn gen_fib(n: u32) -> u32 {
        let mut i = 3;
        let mut current_fib_number: u32 = 1;
        let mut previous_fib_number: u32 = 1;
        while i < n {
            i += 1;
            current_fib_number = current_fib_number + previous_fib_number;
            previous_fib_number = current_fib_number - previous_fib_number;
        }

        current_fib_number
    }

    println!("{}", gen_fib(10));

    // Print the lyrics of The Twelve Days of Christmas
    fn print_twelve() -> String {
        unimplemented!()
    }

    println!("{}", f_to_c(32));
    println!("{}", c_to_f(0));
}
