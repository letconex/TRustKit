# fxt

**fxt** is a command-line tool for local machine translation using [Firefox Translate](https://github.com/mozilla/firefox-translations) models. It runs entirely offline on your machine, requiring no internet connection or API keys.

## Features

- **Offline translation** — uses local NVIDIA-style INT-GEMM models from Firefox Translate
- **Automatic model variant selection** — tries `base`, `base-memory`, and `tiny` model folders in order of preference
- **Pivot translation** — if no direct model exists for a language pair, it automatically translates through English as an intermediate step
- **Simple CLI** — translate text with a single command

## Installation

### Prerequisites

- Rust 1.75+ (for edition 2024)
- Firefox Translate models downloaded locally

### Build from source

```bash
cargo build --release
```

The binary will be available at `target/release/fxt`.

### Setting up models

Download Firefox Translate models and place them in a directory with the following structure:

```
models/
├── base/
│   └── enro/
│       ├── model.enro.intgemm.alphas.bin
│       └── vocab.enro.spm
├── base-memory/
│   └── enro/
│       └── ...
├── tiny/
│   └── enro/
│       └── ...
```

Models can also be placed directly under the models root without a variant folder.

## Usage

```
fxt [OPTIONS] --srclang <SRCLANG> --tgtlang <TGTLANG> --input <INPUT>
```

### Options

| Option | Short | Description |
|---|---|---|
| `--model-path <PATH>` | `-m` | Path to the models directory (default: `D:\FirefoxTranslateLocallyModels\FirefoxModels\models`) |
| `--srclang <CODE>` | `-s` | Source language code (e.g. `en`, `de`, `ro`) |
| `--tgtlang <CODE>` | `-t` | Target language code (e.g. `ro`, `en`, `de`) |
| `--input <TEXT>` | `-i` | Text to translate |

### Examples

**Translate from English to Romanian:**

```bash
fxt -s en -t ro -i "Hello, how are you?"
```

**Translate from German to English with a custom model path:**

```bash
fxt -m "C:\Models" -s de -t en -i "Wie geht es dir?"
```

**Translate from Romanian to German (via English pivot):**

If no direct `rode` model is available, `fxt` will automatically translate Romanian → English → German:

```bash
fxt -s ro -t de -i "Buna ziua, cum va merge?"
```

## How It Works

1. **Direct translation** — `fxt` first looks for a model that covers the source→target language pair directly (e.g. `enro` for English→Romanian).
2. **Model variant search** — for each language pair, it checks the `base`, `base-memory`, and `tiny` subfolders in order, then falls back to the root models directory.
3. **Pivot translation** — if no direct model exists and neither language is English, it translates through English as an intermediate step (source → English → target).

## License

This project is licensed under the same terms as the Firefox Translate models it uses.
