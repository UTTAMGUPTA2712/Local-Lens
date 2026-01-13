use std::sync::{Arc, Mutex, mpsc, atomic::{AtomicBool, Ordering}};
use ort::session::Session;
use crate::app::AppMessage;
use crate::{db, ml, scanner, ocr};

pub fn run_tagging_process(
    folder: String,
    session: Arc<Mutex<Session>>,
    labels: Arc<Vec<ml::Label>>,
    ocr: Arc<ocr::OcrModel>,
    sender: mpsc::Sender<AppMessage>,
    token: Arc<AtomicBool>,
) {
    let images = scanner::scan_images(&folder);
    let total = images.len();
    sender.send(AppMessage::Log(format!("Found {} images.", total))).ok();
    
    // Open separate DB connection for this thread
    let db_conn = match db::setup_db() {
        Ok(conn) => conn,
        Err(e) => {
            sender.send(AppMessage::Log(format!("Failed to open DB: {}", e))).ok();
            return;
        }
    };
    
    for (i, img) in images.iter().enumerate() {
        // Check cancellation
        if token.load(Ordering::Relaxed) {
            sender.send(AppMessage::Log("Tagging cancelled by user.".to_string())).ok();
            sender.send(AppMessage::Finished).ok();
            return;
        }

        sender.send(AppMessage::Progress(i + 1, total)).ok();
        
        if scanner::is_already_tagged(img) {
            sender.send(AppMessage::Log(format!("Skipping {:?} (already tagged)", img.file_name().unwrap_or_default()))).ok();
            continue;
        }

        sender.send(AppMessage::Log(format!("Processing {:?}", img.file_name().unwrap_or_default()))).ok();
        
        // Scope for mutex lock
        {
            if let Ok(mut session_guard) = session.lock() {
                match ml::generate_tags(&mut *session_guard, &labels, img) {
                    Ok(mut tags) => {
                            // OCR Extraction
                            if let Ok(ocr_text) = ocr.extract_text(img) {
                                for word in ocr_text {
                                    if !tags.contains(&word) {
                                        tags.push(word);
                                    }
                                }
                            }
                            
                            // 1. Embed metadata
                            if let Err(e) = scanner::embed_tags_metadata(img, &tags) {
                            sender.send(AppMessage::Log(format!("Error embedding tags: {}", e))).ok();
                            }
                            
                            // 2. Store in DB
                            if let Err(e) = db::store_tags(&db_conn, img, &tags) {
                                sender.send(AppMessage::Log(format!("Error saving to DB: {}", e))).ok();
                            } 
                    }
                    Err(e) => {
                        sender.send(AppMessage::Log(format!("Error generating tags: {}", e))).ok();
                    }
                }
            } else {
                    sender.send(AppMessage::Log("Failed to lock session".to_string())).ok();
                    break;
            }
        }
    }
    sender.send(AppMessage::Log("Tagging complete.".to_string())).ok();
    sender.send(AppMessage::Finished).ok();
}

pub fn run_renaming_process(
    old_tag: String,
    new_tag: String,
    sender: mpsc::Sender<AppMessage>,
) {
    let db_conn = match db::setup_db() {
        Ok(c) => c,
        Err(e) => {
            sender.send(AppMessage::Log(format!("DB Error: {}", e))).ok();
            sender.send(AppMessage::Finished).ok();
            return;
        }
    };
    
    match db::get_images_with_tag(&db_conn, &old_tag) {
        Ok(images) => {
            let total = images.len();
            sender.send(AppMessage::Log(format!("Found {} images with tag '{}'", total, old_tag))).ok();
            
            for (i, (path, mut tags)) in images.into_iter().enumerate() {
                sender.send(AppMessage::Progress(i + 1, total)).ok();
                
                // Precise replacement
                if let Some(pos) = tags.iter().position(|t| t == &old_tag) {
                    tags[pos] = new_tag.clone();
                    // Deduplicate if new tag already existed
                    tags.sort();
                    tags.dedup();
                    
                    // Update DB
                    if let Err(e) = db::store_tags(&db_conn, &path, &tags) {
                            sender.send(AppMessage::Log(format!("DB Update Error: {}", e))).ok();
                    }
                    
                    // Update File
                    if let Err(e) = scanner::embed_tags_metadata(&path, &tags) {
                        sender.send(AppMessage::Log(format!("Metadata Error {:?}: {}", path.file_name(), e))).ok();
                    }
                }
            }
        }
        Err(e) => {
            sender.send(AppMessage::Log(format!("Search Error: {}", e))).ok();
        }
    }
    
    sender.send(AppMessage::Log("Renaming complete.".to_string())).ok();
    sender.send(AppMessage::Finished).ok();
}
