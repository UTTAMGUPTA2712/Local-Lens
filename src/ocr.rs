use oar_ocr::oarocr::{OAROCRBuilder, OAROCR};
use oar_ocr::utils::load_image;
use std::path::{Path, PathBuf};

pub struct OcrModel {
    engine: Option<OAROCR>,
}

impl OcrModel {
    pub fn new() -> Self {
        let det_path = PathBuf::from("models/det_model.onnx");
        let rec_path = PathBuf::from("models/rec_model.onnx");
        let keys_path = PathBuf::from("models/en_dict.txt");

        if !det_path.exists() || !rec_path.exists() || !keys_path.exists() {
            eprintln!("OCR models/keys not found (checked: {:?}, {:?}, {:?}). Text extraction disabled.", det_path, rec_path, keys_path);
            return Self { engine: None };
        }

        // Initialize Builder
        let build_result = OAROCRBuilder::new(
            det_path.to_str().unwrap(),
            rec_path.to_str().unwrap(),
            keys_path.to_str().unwrap()
        ).build();

        match build_result {
            Ok(engine) => Self { engine: Some(engine) },
            Err(e) => {
                eprintln!("Failed to initialize OCR engine: {}", e);
                Self { engine: None }
            }
        }
    }

    pub fn extract_text(&self, path: &Path) -> anyhow::Result<Vec<String>> {
        if let Some(engine) = &self.engine {
            // Load image using oar-ocr utils
            let img = load_image(path)?;
            
            // Run OCR (predict takes a batch, we send 1)
            let results = engine.predict(vec![img])?;
            
            let mut words = Vec::new();
            for result in results {
                for region in result.text_regions {
                     if let Some(text) = region.text {
                        for word in text.split_whitespace() {
                            let cleaned: String = word.chars()
                                .filter(|c| c.is_alphanumeric())
                                .collect();
                            if cleaned.len() > 2 {
                                words.push(cleaned.to_lowercase());
                            }
                        }
                     }
                }
            }
            words.sort();
            words.dedup();
            Ok(words)
        } else {
            Ok(Vec::new())
        }
    }
}
