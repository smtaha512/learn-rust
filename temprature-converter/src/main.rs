use std::io;

fn main() {
    println!("Temprature Converter:");

    let temp = loop {
        println!("Enter temprature:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temprature");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Invalid input. ");
                continue;
            }
        };
        break temp;
    };

    let unit = loop {
        println!("Enter temprature unit `C` or `F`");

        let mut unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read unit");

        let unit: char = match unit.trim().parse() {
            Ok(str) => str,
            Err(_) => {
                print!("Invalid input. ");
                continue;
            }
        };

        if !is_unit_allowed(unit) {
            print!("Invalid input. ");
            continue;
        }

        break unit;
    };

    let converted_temp: f64 = if is_fahrenheit(unit) {
        (5.0 / 9.0) * (temp - 32.0)
    } else {
        ((9.0 / 5.0) * (temp)) + 32.0
    };

    print!("Converted temprature is {} : ", converted_temp);
}

fn is_celsius(unit: char) -> bool {
    const C_UNITS: [char; 2] = ['C', 'c'];
    C_UNITS.contains(&unit)
}

fn is_fahrenheit(unit: char) -> bool {
    const F_UNITS: [char; 2] = ['F', 'f'];
    F_UNITS.contains(&unit)
}

fn is_unit_allowed(unit: char) -> bool {
    is_celsius(unit) || is_fahrenheit(unit)
}
