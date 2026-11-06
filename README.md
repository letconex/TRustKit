# TSV2DSL CLI

`tsv2dsl` is a command-line utility that converts a TSV dictionary file into a GoldenDict / Abby Lingvo DSL dictionary file.

It reads a tab-separated file where each line contains a source term and a target term, then writes a DSL file with the required header metadata and entry format used by dictionary readers such as GoldenDict.

## What it does

The app accepts an input TSV file like this:

```tsv
Hallo	Hello
Wasser	Water
Karte	Map
```

And converts it to a DSL file in the same directory as the input file, using the `.dsl` extension. The output format looks like this:

```dsl
#NAME "DE-EN"
#INDEX_LANGUAGE "German"
#CONTENTS_LANGUAGE "English"

Hallo
	Hello

Wasser
	Water

Karte
	Map
```

This structure is compatible with dictionary software that expects DSL entries in the form:

```text
source text
	target text
```

## Features

- Converts TSV entries to DSL entries
- Rewrites output to the same folder as the source file
- Supports dictionary metadata headers:
  - `#NAME`
  - `#INDEX_LANGUAGE`
  - `#CONTENTS_LANGUAGE`
- Supports optional target text color
- Supports optional entry formatting such as bold, italic, underline, and indentation styles
- Validates input to reject malformed lines or empty files
- Handles UTF-8 BOM safely

## Command usage

```bash
tsv2dsl INPUT.tsv --name "DE-RO" --source "German" --target "Romanian"
```

### Common options

- `-n, --name <NAME>`: dictionary name written to the `#NAME` header
- `-s, --source <SOURCE>`: source language written to `#INDEX_LANGUAGE`
- `-t, --target <TARGET>`: target language written to `#CONTENTS_LANGUAGE`
- `-c, --colour [COLOUR]`: optional color for target entries, such as `darkblue`, `darkgreen`, or `teal`
- `-f, --format <FORMAT>`: optional formatting flags, such as `bold`, `italic`, `underline`, `m1`, `m2`, or `m3`

## Input requirements

The input file must be UTF-8 text with one entry per line and exactly one tab separator between the source and target text:

```tsv
source<TAB>target
```

Examples:

```tsv
haus	house
gehen	to go
```

The app rejects:

- blank lines
- lines without a tab separator
- lines with more than one tab separator
- empty source or target fields
- files with no meaningful content

## Formatting options

The app supports formatting tags for the target text. These are applied to the translated entry:

- `b` or `bold`
- `i` or `italic`
- `u` or `underline`
- `m1`, `m2`, `m3` for indentation styles

Formatting values are normalized and validated. Only one indentation variant may be used at a time, and formatter order is enforced.

Example:

```bash
tsv2dsl dictccdero.tsv -n "DE-RO" -s "German" -t "Romanian" -c darkgreen -f bold -f italic -f m1
```

## Output location

The generated `.dsl` file is stored in the same directory as the input TSV file, using the same base filename.

Example:

```text
input.tsv  ->  input.dsl
```

## Example

```bash
tsv2dsl dictionary.tsv --name "EN-FR" --source "English" --target "French" --colour darkblue --format bold --format m1
```

This creates a file such as:

```text
dictionary.dsl
```

with dictionary metadata and entries ready for import into GoldenDict or Abby Lingvo-compatible dictionary software.

## Notes

- The app writes a UTF-8 BOM at the start of the generated DSL file.
- The output is designed for dictionary readers that support DSL formatting tags.
- Errors are printed to stderr and the program exits with a non-zero status if invalid input is provided.
