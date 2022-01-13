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
        match n {
            0 => 0,
            1 => 1,
            _ => gen_fib(n - 1) + gen_fib(n - 2),
        }
    }

    // Print the lyrics of The Twelve Days of Christmas
    //
    fn gen_song() -> String {
        let mut lyrics: Vec<String> = Vec::new();
        for verse in 0..=11 {
            lyrics.push(gen_verse(verse));
            lyrics.push("\n\n".to_string())
        }
        lyrics.concat()
    }

    println!("{}", gen_song());

    fn gen_verse(verse_index: u8) -> String {
        let mut verse: Vec<String> = vec![verse_boilerplate(verse_index), "\n".to_string()];
        for day in 0..=verse_index {
            verse.insert(2, format!("{}\n", verse_item(day)))
        }
        verse.concat()
    }

    enum DayOfChristmas {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
        Sixth,
        Seventh,
        Eighth,
        Ninth,
        Tenth,
        Eleventh,
        Twelfth,
    }

    fn verse_boilerplate(verse_index: u8) -> String {
        let convert = |x| match x {
            0 => "first",
            1 => "second",
            2 => "third",
            3 => "fourth",
            4 => "fifth",
            5 => "sixth",
            6 => "seventh",
            7 => "eighth",
            8 => "ninth",
            9 => "tenth",
            10 => "eleventh",
            11 => "twelfth",
            _ => "ERROR",
        };
        format!(
            "On the {} day of Christmas, my true love gave to me",
            convert(verse_index)
        )
    }

    fn verse_item(verse_index: u8) -> String {
        match verse_index {
            0 => "A partridge in a pear tree",
            1 => "Two turtle doves, and",
            2 => "Three french hens,",
            3 => "Four calling birds,",
            4 => "Five golden rings,",
            5 => "Six geese a-laying,",
            6 => "Seven swans a-swimming,",
            7 => "Eight maids a-milking,",
            8 => "Nine ladies dancing,",
            9 => "Ten lords a-leaping,",
            10 => "Eleven pipers piping,",
            11 => "Twelve drummers drumming,",
            _ => "ERROR",
        }
        .to_string()
    }

    // On the twelfth day of Christmas, my true love sent to me
    // Twelve drummers drumming
    // Eleven pipers piping
    // Ten lords a-leaping
    // Nine ladies dancing
    // Eight maids a-milking
    // Seven swans a-swimming
    // Six geese a-laying
    // Five golden rings
    // Four calling birds
    // Three french hens
    // Two turtle doves, and
    // A partridge in a pear tree

    println!("{}", f_to_c(32));
    println!("{}", c_to_f(0));
    println!("{}", gen_fib(10));
}
