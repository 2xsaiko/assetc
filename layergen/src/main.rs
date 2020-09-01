use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use clap::{app_from_crate, Arg};
use image::{ImageBuffer, RgbaImage, Rgba, ImageFormat};
use serde::{Deserialize, Deserializer};
use serde::de::Error;

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("image-src").short('I').value_name("DIR").multiple_occurrences(true))
        .arg(Arg::with_name("colormap").short('m').value_name("FILE").required(true))
        .arg(Arg::with_name("output").short('o').value_name("FILE").default_value("a.png"))
        .arg(Arg::with_name("image").value_name("IMAGE").required(true))
        .get_matches();

    let image_src = matches.values_of_os("image-src").map(|v| v.map(Path::new).collect()).unwrap_or_else(|| Vec::new());
    let colormap = Path::new(matches.value_of_os("colormap").unwrap());
    let output = Path::new(matches.value_of_os("output").unwrap());
    let image = Path::new(matches.value_of_os("image").unwrap());

    let map: ColorMap = serde_yaml::from_reader(File::open(colormap).expect("Failed to open color map")).expect("Failed to read color map");
    let fallback = map.0.iter().filter_map(|a| if a.fallback.unwrap_or(false) { Some(a) } else { None }).next();

    let image = image::open(image).expect("Failed to open input image").into_rgb();

    let mut images: HashMap<&Path, RgbaImage> = HashMap::new();
    for entry in map.0.iter() {
        if let ColorSource::Image(p) = &entry.source {
            if !images.contains_key(&**p) {
                let mut i = None;
                for &dir in image_src.iter() {
                    if let Ok(img) = image::open(dir.join(p)) {
                        i = Some(img.into_rgba());
                        break;
                    }
                }

                match i {
                    None => {
                        panic!("Failed to open referenced image {}", p.to_string_lossy());
                    }
                    Some(i) => {
                        if i.width() != image.width() || i.height() != image.height() {
                            panic!("Included images must be same size");
                        }
                        images.insert(p, i);
                    }
                }
            }
        }
    }

    let mut output_image: RgbaImage = ImageBuffer::new(image.width(), image.height());

    for i in 0..image.width() {
        for j in 0..image.height() {
            let pixel = image.get_pixel(i, j);
            let [r, g, b] = pixel.0;
            let color = Color { a: 0xff, r, g, b };

            let mut entry = None;

            for e in map.0.iter() {
                if let Some(c) = e.color {
                    if c == color {
                        entry = Some(e);
                        break;
                    }
                }
            }

            entry = entry.or(fallback);

            match entry {
                None => panic!("No rule found for image color #{:02x}{:02x}{:02x}", r, g, b),
                Some(entry) => {
                    let mut color = match &entry.source {
                        ColorSource::Fill(c) => *c,
                        ColorSource::Image(p) => {
                            let [r, g, b, a] = images.get(&**p).unwrap().get_pixel(i, j).0;
                            Color { r, g, b, a }
                        }
                    };

                    for filter in entry.filters.iter() {
                        match filter {
                            ColorFilter::Multiply { color: c } => {
                                color = color.multiply(*c);
                            }
                        }
                    }

                    output_image.put_pixel(i, j, Rgba([color.r, color.g, color.b, color.a]));
                }
            }
        }
    }

    output_image.save_with_format(output, ImageFormat::Png).unwrap();
}

#[derive(Debug, Deserialize)]
struct ColorMap(Vec<ColorMapEntry>);

#[derive(Debug, Deserialize)]
struct ColorMapEntry {
    color: Option<Color>,
    fallback: Option<bool>,
    #[serde(default)]
    filters: Vec<ColorFilter>,
    #[serde(flatten)]
    source: ColorSource,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Color { r: u8, g: u8, b: u8, a: u8 }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ColorSource {
    Fill(Color),
    Image(PathBuf),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
enum ColorFilter {
    Multiply {
        color: Color,
    }
}

impl Color {
    fn from_rgba(color: u32) -> Color {
        let [a, r, g, b] = color.to_be_bytes();
        Color { r, g, b, a }
    }

    fn from_rgb(color: u32) -> Color {
        Color::from_rgba(0xff000000 | color)
    }

    fn from_hex_string(s: &str) -> Result<Color, ()> {
        if s.len() == 8 {
            u32::from_str_radix(s, 16).map(Color::from_rgba).map_err(|_| ())
        } else if s.len() == 6 {
            u32::from_str_radix(s, 16).map(Color::from_rgb).map_err(|_| ())
        } else {
            Err(())
        }
    }

    fn multiply(self, other: Color) -> Color {
        Color {
            r: dmul255(self.r, other.r),
            g: dmul255(self.g, other.g),
            b: dmul255(self.b, other.b),
            a: dmul255(self.a, other.a),
        }
    }
}

fn dmul255(a: u8, b: u8) -> u8 { (((a as f32 / 255.0) * (b as f32 / 255.0)) * 255.0) as u8 }

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("#") {
            Color::from_hex_string(&s[1..]).map_err(|_| D::Error::custom("invalid hex syntax"))
        } else {
            match &*s {
                "transparent" => Ok(Color::from_rgba(0x00000000)),
                _ => Err(D::Error::custom("invalid color spec"))
            }
        }
    }
}