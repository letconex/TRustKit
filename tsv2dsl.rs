#![allow(dead_code)]
use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
mod res;
use crate::res::*;

/// Convert a TSV dictionary file to a GoldenDict / Abby Lingvo DSL file.
///
/// Example: cargo run -- dictccdero.tsv  --name "DE-RO" --source "German" --target "Romanian" -c teal -f b - f m1
///
/// Input TSV format:
/// source text<TAB>target text<EOL>
///
/// Output DSL format:
/// #NAME "..."
/// #INDEX_LANGUAGE "..."
/// #CONTENTS_LANGUAGE "..."
/// <EOL>
/// source text<EOL>
/// <TAB>target text<EOL>
/// ...
/// Possible formatting options: bold, italic, underline, indentation m1, m2, m3
#[derive(Parser, Debug)]
#[command(
    author = "letconex, https://github/letconex",
    version,
    about = "TSV to DSL Converter",
    long_about = "Convert a TSV dictionary file to a GoldenDict DSL file",
    after_help = r#"
Convert a TSV file to Abby/Goldendict file in the same directory as input.
The output will be stored in the same directory as the input file with the name provided by -n or --name argument.
The TSV format should contain two columns separated using TAB separator and one line per entry containing source text followed immediately after target texr formatted like this:
source<TAB>target<EOL>
Target language text colour can also optionally passed in the -c or --colour argument. If not specified, default is blue.
Example:
tsv2dsl dictccdero.tsv -n "DE-RO" -s "German" -t "Romanian" -c darkgreen
tsv2dsl dictccdero.tsv -n "DE-RO" -s "German" -t "Romanian" -c // Default colour is blue.
tsv2dsl dictccdero.tsv -n "DE-RO" -s "German" -t "Romanian" // No colour
"#
)]
struct Args {
    /// Input TSV file path
    #[arg(value_name = "INPUT TSV FILE")]
    input: PathBuf,

    /// Dictionary name used in the DSL #NAME header
    #[arg(short, long)]
    name: String,

    /// Source language used in #INDEX_LANGUAGE
    #[arg(short, long)]
    source: String,

    /// Target language used in #CONTENTS_LANGUAGE
    #[arg(short, long)]
    target: String,

    /// Colour for target language default_value = "" default_missing_value = "darkblue"
    #[arg(short, long, default_missing_value = "darkblue", num_args(0..=1))]
    // colour: String,
    colour: Option<String>,

    /// Formatting options: bold, underline and italics for text. M1, M2, M3 for indentation.
    #[arg(long, short, value_enum)]
    format: Vec<Formatter>,
}

fn convert_tsv_to_dsl_atomic(
    input_path: &Path,
    output_path: &Path,
    name: &str,
    source: &str,
    target: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = output_path.with_extension("dsl.tmp");
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let temp_file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(temp_file);

    // write BOM + headers
    writer.write_all(b"\xEF\xBB\xBF")?;
    write!(writer, "#NAME \"{}\"\r\n", escape_header_value(name))?;
    write!(
        writer,
        "#INDEX_LANGUAGE \"{}\"\r\n",
        escape_header_value(source)
    )?;
    write!(
        writer,
        "#CONTENTS_LANGUAGE \"{}\"\r\n",
        escape_header_value(target)
    )?;
    write!(writer, "\r\n")?;

    // process lines; any error -> remove temp and return Err
    for (line_number, line_result) in (1..).zip(reader.lines()) {
        let line = line_result?;
        let (source_text, target_text) = parse_tsv_line(line_number, &line)?;
        writer.write_all(source_text.as_bytes())?;
        writer.write_all(b"\r\n\t")?;
        writer.write_all(target_text.as_bytes())?;
        writer.write_all(b"\r\n")?;
    }

    writer.flush()?;
    // atomically replace
    fs::rename(&temp_path, output_path)?;
    Ok(())
}

fn convert_tsv_to_dsl(
    reader: BufReader<File>,
    output_path: &Path,
    name: &str,
    source: &str,
    target: &str,
    colour: &str,
    format: &[Formatter],
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input and get reader
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);

    // UTF-8 BOM: EF BB BF
    writer.write_all(b"\xEF\xBB\xBF")?;

    // DSL header
    write!(writer, "#NAME \"{}\"\r\n", escape_header_value(name))?;
    write!(
        writer,
        "#INDEX_LANGUAGE \"{}\"\r\n",
        escape_header_value(source)
    )?;
    write!(
        writer,
        "#CONTENTS_LANGUAGE \"{}\"\r\n",
        escape_header_value(target)
    )?;
    write!(writer, "\r\n")?;

    // Process TSV line by line.
    for (line_number, line_result) in (1..).zip(reader.lines()) {
        let line: String = line_result?;
        let (source_text, target_text) = parse_tsv_line(line_number, &line)?;
        let target_text = if colour.is_empty() {
            target_text.to_string()
        } else {
            format!("[c {colour}]{target_text}[/c]")
        };
        /* DSL entry:
        source\r\n
        \ttarget\r\n */
        let target_text = apply_formatting(&target_text, &format);
        writer.write_all(source_text.as_bytes())?;
        writer.write_all(b"\r\n\t")?;
        writer.write_all(target_text.as_bytes())?;
        writer.write_all(b"\r\n")?;
    }

    writer.flush()?;

    // Make sure the file exists and is readable after writing.
    fs::metadata(output_path)?;

    Ok(())
}

/// Escape characters that could interfere with the quoted DSL header value.
fn escape_header_value(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn parse_tsv_line(
    line_number: usize,
    line: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Remove BOM only on first line, a possible UTF-8 BOM from the beginning of the input TSV.
    let line = if line_number == 1 {
        line.strip_prefix('\u{FEFF}').unwrap_or(line)
    } else {
        line
    };

    // Ignore empty lines
    if line.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Line {} is empty", line_number),
        )
        .into());
    }

    let tabs_count = line.matches('\t').count();

    match tabs_count {
        0 => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Line {} contains no tabs: {}", line_number, line),
            )
            .into());
        }
        1 => {
            let (source, target) = line.split_once('\t').ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {}: expected one tab", line_number),
                )
            })?;

            let source_text = source.trim().to_string();
            let target_text = target.trim().to_string();

            if source_text.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {}: source text is empty", line_number),
                )
                .into());
            }
            if target_text.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Line {}: target text is empty", line_number),
                )
                .into());
            }
            Ok((source_text, target_text))
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Line {} contains more than one tab: {}", line_number, line),
            )
            .into());
        }
    }
}

fn open_nonempty_reader(input_path: &Path) -> io::Result<BufReader<File>> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    let mut bytes_consumed = 0;
    let mut has_content = false;

    loop {
        let buf = reader.fill_buf()?;
        if buf.is_empty() {
            break; // Reached End-Of-File
        }

        // Skip 3-byte UTF-8 BOM (0xEF, 0xBB, 0xBF) if present at start of file
        let start_offset = if bytes_consumed == 0 && buf.starts_with(&[0xEF, 0xBB, 0xBF]) {
            3
        } else {
            0
        };

        // Scan buffer slice for any non-whitespace byte
        if buf[start_offset..].iter().any(|b| !b.is_ascii_whitespace()) {
            has_content = true;
            break;
        }

        let len = buf.len();
        bytes_consumed += len;
        reader.consume(len);
    }

    if !has_content {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Input file '{}' contains no meaningful content",
                input_path.display()
            ),
        ));
    }

    // Rewind only if we advanced the read position past the first buffer window
    if bytes_consumed > 0 {
        reader.seek(SeekFrom::Start(0))?;
    }

    Ok(reader)
}

fn main() {
    let args: Args = Args::parse();
    let formats: Vec<Formatter> = parse_formatters(&args.format);
    dbg!(&args);
    // println!("{:?} {:?}", args, formats);

    let input_path: PathBuf = args.input;
    let output: PathBuf = input_path.with_extension("dsl");
    // let colour = parse_colours_ignore(args.colour);
    // dbg!(&colour);
    let colour: Option<String> = match parse_colour(args.colour) {
        Ok(colour) => colour,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let reader = match open_nonempty_reader(&input_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = convert_tsv_to_dsl(
        reader,
        &output,
        &args.name,
        &args.source,
        &args.target,
        colour.as_deref().unwrap_or(""),
        &formats,
    ) {
        // if let Err(e) =
        // convert_tsv_to_dsl_atomic(&input_path, &output, &args.name, &args.source, &args.target) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!(
        "Converted '{}' -> '{}'",
        input_path.display(),
        output.display()
    );
}
