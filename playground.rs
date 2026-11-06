fn open_nonempty_readerc(input_path: &Path) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    loop {
        let bytes = reader.read_line(&mut line)?;

        if bytes == 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Input file '{}' contains no meaningful lines",
                    input_path.display()
                ),
            )
            .into());
        }

        let trimmed = line.trim();

        // Skip BOM-only line
        if trimmed == "\u{FEFF}" {
            line.clear();
            continue;
        }

        // Skip whitespace-only lines
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        break; // Found meaningful content
    }

    // Rewind so caller reads from the beginning
    reader.seek(SeekFrom::Start(0))?;

    Ok(reader)
}
