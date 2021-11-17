use embedded_graphics::{
    egtext,
    fonts::{Font12x16, Font24x32, Text},
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
    text_style,
};
use epd_waveshare::epd4in2::*;

use arrayvec::ArrayString;
use core::fmt::Write;

pub fn draw_text(mut display: Display4in2) -> Display4in2 {
    Text::new("Air Quality", Point::new(20, 30))
        .into_styled(TextStyle::new(Font24x32, BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    Text::new("Carbon Dioxide:", Point::new(20, 90))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    Text::new("Temperature:", Point::new(20, 130))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    Text::new("Humidity:", Point::new(20, 170))
        .into_styled(TextStyle::new(Font12x16, BinaryColor::On))
        .draw(&mut display)
        .unwrap();

    display
}

pub fn draw_numbers(value: f32, unit: &str, position: (i32, i32), mut display: Display4in2) -> Display4in2 {
    let mut buf = ArrayString::<{ 12 }>::new();
    write!(&mut buf, "{:.2} {}", value, unit).expect("Failed to write to buffer");

    egtext!(
        text = &buf,
        top_left = position,
        style = text_style!(
            font = Font12x16,
            text_color = BinaryColor::On,
        )
    )
    .draw(&mut display).unwrap();
    
    display
} 
