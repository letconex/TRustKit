use clap::Parser;
use fxtranslate::engine::Engine;
use std::path::{Path, PathBuf};

/// Model variant folders in order of preference.
#[derive(Debug, Clone, Copy)]
enum ModelVariant {
    Base,
    BaseMemory,
    Tiny,
}

impl ModelVariant {
    const ALL: [ModelVariant; 3] = [ModelVariant::Base, ModelVariant::BaseMemory, ModelVariant::Tiny];

    fn folder_name(self) -> &'static str {
        match self {
            ModelVariant::Base => "base",
            ModelVariant::BaseMemory => "base-memory",
            ModelVariant::Tiny => "tiny",
        }
    }
}

/// Translate text using Firefox Translate local models.
#[derive(Parser)]
#[command(name = "fxt", about = "Local machine translation with fxtranslate using Firefox local translation models")]
struct Args {
    /// Path to the models directory (e.g. D:\Models)
    #[arg(short, long, default_value = r"D:\FirefoxTranslateLocallyModels\FirefoxModels\models")]
    model_path: PathBuf,

    /// Source language code (e.g. "en", "de")
    #[arg(short, long)]
    srclang: String,

    /// Target language code (e.g. "ro", "en")
    #[arg(short = 't', long)]
    tgtlang: String,

    /// Text to translate
    #[arg(short, long)]
    input: String,
}

fn find_model(
    main_path: &PathBuf,
    lang_pair: &str,
) -> Option<(String, String)> {
    // First try variant subfolders in preference order
    for variant in ModelVariant::ALL {
        let folder = variant.folder_name();
        let model_path = format!("{}/{folder}/{lang_pair}/model.{lang_pair}.intgemm.alphas.bin", main_path.display());
        let vocab_path = format!("{}/{folder}/{lang_pair}/vocab.{lang_pair}.spm", main_path.display());
        if Path::new(&model_path).exists() && Path::new(&vocab_path).exists() {
            #[cfg(debug_assertions)]
            eprintln!("Found model in '{}' variant: {}", folder, model_path);
            return Some((model_path, vocab_path));
        }
    }
    // Fallback: check directly under main_path (for models not in a variant folder)
    let model_path = format!("{}/{lang_pair}/model.{lang_pair}.intgemm.alphas.bin", main_path.display());
    let vocab_path = format!("{}/{lang_pair}/vocab.{lang_pair}.spm", main_path.display());
    if Path::new(&model_path).exists() && Path::new(&vocab_path).exists() {
        eprintln!("Found model directly under main path: {}", model_path);
        return Some((model_path, vocab_path));
    }
    None
}

fn load_engine(main_path: &PathBuf, lang_pair: &str) -> Result<Engine, String> {
    let (model_path, vocab_path) = find_model(main_path, lang_pair)
        .ok_or_else(|| format!("No model found for language pair '{}'", lang_pair))?;
    #[cfg(debug_assertions)]
    eprintln!("Loading model: {}", model_path);
    Engine::load(&model_path, &vocab_path, &vocab_path)
}

/// Translate text using the given arguments.
///
/// Tries direct translation first, then falls back to pivot through English.
fn translate(from: &str, to: &str, text: &str, model_path: &PathBuf) -> Result<String, String> {
    // Direct translation: source→target model exists
    let lang_pair = format!("{}{}", from, to);
    if let Ok(engine) = load_engine(model_path, &lang_pair) {
        #[cfg(debug_assertions)]
        eprintln!("Direct translation: {} → {}", from, to);
        return Ok(engine.translate(text));
    }

    // Pivot translation through English: source→en→target
    let pivot = "en";
    if from != pivot && to != pivot {
        let first_leg = format!("{}{}", from, pivot);
        let second_leg = format!("{}{}", pivot, to);

        let engine1 = load_engine(model_path, &first_leg)
            .map_err(|e| format!(
                "Cannot translate {} → {}. No direct model, and pivot via '{}' failed: {}. Checked models: {} and {}",
                from, to, pivot, e, first_leg, second_leg
            ))?;
        let engine2 = load_engine(model_path, &second_leg)
            .map_err(|e| format!(
                "Cannot translate {} → {}. No direct model, and pivot via '{}' failed: {}. Checked models: {} and {}",
                from, to, pivot, e, first_leg, second_leg
            ))?;

        #[cfg(debug_assertions)]
        eprintln!("Pivot translation: {} → {} → {}", from, pivot, to);
        let intermediate = engine1.translate(text);
        #[cfg(debug_assertions)]
        eprintln!("Intermediate translation from/to English ({}): {}", pivot, intermediate);
        return Ok(engine2.translate(&intermediate));
    }

    // No direct model and cannot pivot (one side is already English)
    Err(format!(
        "No model found for language pair '{}' in any variant (base, base-memory, tiny) under '{}'.",
        lang_pair, model_path.display()
    ))
}

fn main() {
    let args: Args = Args::parse();
    let from: &String = &args.srclang;
    let to: &String = &args.tgtlang;
    let text: &String = &args.input;
    let model_path = &args.model_path;

    match translate(from, to, text, model_path) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
