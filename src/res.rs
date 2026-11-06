#![allow(dead_code)]
use std::collections::HashSet;
use std::fmt;
use std::path::PathBuf;

/// A constant with possible colour values. Default value: darkblue
const COLOURS: &[&str] = &[
    "aliceblue",
    "antiquewhite",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "green",
    "greenyellow",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgreen",
    "lightgrey",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "magenta",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "turquoise",
    "violet",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
];

/// Function to check if a string is a valid colour.
///
/// Returns an error if the string is not a valid colour, or a warning if the string is invalid, but can be resolved.
///
/// # Arguments
///
/// * `colour` - The string slice to check.
///
/// # Examples
///
/// ```
/// let colour: Option<String> = "red"; {
///   Ok(colour) => colour,
///   Err(error) => {
///     eprintln!("Error: {}", error);
///     std::process::exit(1);
///   }
/// };
/// ```
///
/// Returns `Ok(())` if the string is a valid colour, or an error if the string is invalid.
pub fn validate_colour(colour: &str) -> Result<(), String> {
    if COLOURS.contains(&colour.to_ascii_lowercase().as_str()) {
        Ok(())
    } else {
        Err(format!(
            "Invalid colour '{}'. Possible colours: {}.",
            colour,
            COLOURS.join(", ")
        ))
    }
}

/// Parses a colour string into an [`Option<String>`] that is `None` if the string is `None`, or `Some(colour)` if the string is a valid colour.
pub fn parse_colour(colour: Option<String>) -> Result<Option<String>, String> {
    match colour {
        None => Ok(None),
        Some(colour) => {
            validate_colour(&colour)?;
            Ok(Some(colour))
        }
    }
}

fn parse_colour_ignore(colour: Option<String>) -> Option<String> {
    match colour {
        None => None,

        Some(colour) => match validate_colour(&colour) {
            Ok(()) => {
                println!("Valid colour '{}'!", colour);
                Some(colour)
            }
            Err(warning) => {
                eprintln!(
                    "Continuing without target text colour! Warning:\n{}",
                    warning
                );
                None
            }
        },
    }
}

/// Define an enum with multiple variants: Bold, Italic, Underline, indentation M1, M2, M3
/// Each variant represents a formatting option for dictionary entries.
#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum Formatter {
    Bold,
    B,
    Italic,
    I,
    Underline,
    U,
    M1,
    M2,
    M3,
}

impl Formatter {
    /// Returns the order of the formatter in the format string.
    pub fn order(&self) -> usize {
        match self {
            Formatter::Bold | Formatter::B => 1,
            Formatter::Italic | Formatter::I => 2,
            Formatter::Underline | Formatter::U => 3,
            Formatter::M1 => 4,
            Formatter::M2 => 5,
            Formatter::M3 => 6,
        }
    }
}

impl fmt::Display for Formatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formatter::Bold | Formatter::B => write!(f, "Bold"),
            Formatter::Italic | Formatter::I => write!(f, "Italic"),
            Formatter::Underline | Formatter::U => write!(f, "Underline"),
            Formatter::M1 => write!(f, "M1 indentation"),
            Formatter::M2 => write!(f, "M2 indentation"),
            Formatter::M3 => write!(f, "M3 indentation"),
        }
    }
}

/// Remove duplicate formatters
fn deduplicate_formatters(formats: &[Formatter]) -> Vec<Formatter> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut result: Vec<Formatter> = Vec::new();

    for f in formats {
        if seen.insert(f.order()) {
            result.push(*f);
        }
    }
    result
}

/// Function to validate duplicate formatters and their order
fn validate_formatters(formats: &[Formatter]) {
    // 1. Enforce only one m-variant
    let m_count: usize = formats
        .iter()
        .filter(|f| matches!(f, Formatter::M1 | Formatter::M2 | Formatter::M3))
        .count();
    if m_count > 1 {
        eprintln!("Error: Only one of m1, m2 or m3 may be used at a time.");
        std::process::exit(1);
    }
    // 2. Enforce ordering
    for w in formats.windows(2) {
        if w[0].order() > w[1].order() {
            eprintln!(
                "Error: Formatters must be in canonical order: bold → italic → underline → m1/m2/m3"
            );
            std::process::exit(1);
        }
    }
}

/// Deduplicate and validate formatters
pub fn parse_formatters(formats: &[Formatter]) -> Vec<Formatter> {
    let formats: Vec<Formatter> = deduplicate_formatters(formats);
    validate_formatters(&formats);
    formats
}

/// Apply formatting for source text: bold, italic, underline, m1/m2/m3 indentations
pub fn apply_formatting(text: &str, formats: &[Formatter]) -> String {
    let mut result: String = text.to_string();
    for format in formats {
        result = match format {
            Formatter::Bold | Formatter::B => format!("[b]{result}[/b]"),
            Formatter::Italic | Formatter::I => format!("[i]{result}[/i]"),
            Formatter::Underline | Formatter::U => format!("[u]{result}[/u]"),
            Formatter::M1 => format!("[m1]{result}[/m]"),
            Formatter::M2 => format!("[m2]{result}[/m]"),
            Formatter::M3 => format!("[m3]{result}[/m]"),
        };
    }

    result
}

/// Displays a summary of the conversion process.
pub fn display_final(
    formats: Vec<Formatter>,
    colour: Option<String>,
    input_path: PathBuf,
    output: PathBuf,
    source: String,
    target: String,
) {
    let formatting: String = if formats.is_empty() {
        String::from("No formatting!")
    } else {
        formats
            .iter()
            .map(|fmt| fmt.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let colour: String = match colour {
        None => "No colour".to_string(),
        Some(colour) => colour,
    };

    println!(
        "Converted '{}' -> '{}' with source language '{}' and target language '{}'. Colour: '{}'. Formatting: '{}'",
        input_path.display(),
        output.display(),
        source.to_uppercase(),
        target.to_uppercase(),
        colour.to_uppercase(),
        formatting.to_uppercase(),
    );
}
