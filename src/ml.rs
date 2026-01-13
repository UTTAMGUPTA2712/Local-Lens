use image::ImageReader;
use ort::{inputs, session::Session, value::Value};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Label {
    pub name: String,
}

pub fn load_labels() -> anyhow::Result<Vec<Label>> {
    let file = File::open("models/imagenet-simple-labels.json")?;
    let reader = BufReader::new(file);
    let names: Vec<String> = serde_json::from_reader(reader)?;
    
    Ok(names.into_iter().map(|name| Label {
        name,
    }).collect())
}

pub fn generate_tags(
    session: &mut Session,
    labels: &[Label],
    image_path: &Path,
) -> anyhow::Result<Vec<String>> {
    // 1. Preprocess
    let img = ImageReader::open(image_path)?.decode()?;
    // ResNet expects 224x224
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);
    
    // Convert to NCHW [1, 3, 224, 224] and Normalize (Mean=[0.485, 0.456, 0.406], Std=[0.229, 0.224, 0.225])
    let mean = [0.485, 0.456, 0.406];
    let std = [0.229, 0.224, 0.225];
    
    let mut input = Vec::with_capacity(1 * 3 * 224 * 224);
    // Needed plan: R channel, G channel, B channel separately
    let rgb = resized.to_rgb8();
    
    // R channel
    for pixel in rgb.pixels() {
        let val = (pixel[0] as f32 / 255.0 - mean[0]) / std[0]; // R
        input.push(val);
    }
    // G channel
    for pixel in rgb.pixels() {
         let val = (pixel[1] as f32 / 255.0 - mean[1]) / std[1]; // G
         input.push(val);
    }
    // B channel
    for pixel in rgb.pixels() {
        let val = (pixel[2] as f32 / 255.0 - mean[2]) / std[2]; // B
        input.push(val);
    }

    let input_tensor = Value::from_array((vec![1, 3, 224, 224], input.into_boxed_slice()))?;
    
    // 2. Inference
    let outputs = session.run(inputs![input_tensor])?;
    let (_shape, data) = outputs[0].try_extract_tensor::<f32>()?;
    let logits: Vec<f32> = data.to_vec();
    
    // 3. Post-process (Softmax & Top-K)
    // Softmax
    let max_logit = logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let exp_logits: Vec<f32> = logits.iter().map(|&x| (x - max_logit).exp()).collect();
    let sum_exp: f32 = exp_logits.iter().sum();
    let probs: Vec<f32> = exp_logits.iter().map(|&x| x / sum_exp).collect();
    
    // Sort and get Top 5
    let mut scored_labels: Vec<(usize, f32)> = probs.iter().enumerate().map(|(i, &p)| (i, p)).collect();
    scored_labels.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    let top_k = 5;
    let tags: Vec<String> = scored_labels.iter().take(top_k).map(|&(idx, _prob)| {
        if idx < labels.len() {
            labels[idx].name.clone()
        } else {
            format!("class_{}", idx)
        }
    }).collect();

    Ok(tags)
}
