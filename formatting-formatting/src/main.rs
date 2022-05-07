use std::fmt::{self, Display, Formatter, write};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lng: f32,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lng >= 0.0 { 'E' } else { 'W' };
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lng.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/*
Activity

Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:

RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
      
Two hints if you get stuck:

    You may need to list each color more than once,
    You can pad with zeros to a width of 2 with :0>2.
*/
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:0>2X?}{:0>2X?}{:0>2X?}", self.red, self.green, self.blue,
               self.red,
               self
                   .green, self.blue)
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lng: -6.259722 },
        City { name: "Oslo", lat: 59.95, lng: 10.75 },
        City { name: "Vancouver", lat: 49.25, lng: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?} ", *color);
    }

    // Activity
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{} ", *color);
    }
}
