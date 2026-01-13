use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, ProgressBar, ScrollArea, SidePanel, TopBottomPanel, Vec2};
use ort::session::Session;
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc, atomic::{AtomicBool, Ordering}};
use std::thread;

use crate::db;
use crate::ml;
use crate::processing;

pub enum AppMessage {
    Log(String),
    Progress(usize, usize),
    Finished,
}

pub struct ImageTagger {
    folder: String,
    query: String,
    rename_old: String,
    rename_new: String,
    results: Vec<PathBuf>,
    conn: Connection,
    session: Option<Arc<Mutex<Session>>>,
    labels: Arc<Vec<ml::Label>>,
    ocr: Arc<crate::ocr::OcrModel>,
    
    // Threading
    receiver: mpsc::Receiver<AppMessage>,
    sender: mpsc::Sender<AppMessage>, // Keep a clone to give to threads or just clone when spawning
    is_processing: bool,
    progress: (usize, usize),
    logs: Vec<String>,
    cancellation_token: Arc<AtomicBool>,
}

impl ImageTagger {
    pub fn new(cc: &CreationContext) -> Self {
        // Install image loaders
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self::configure_styles(&cc.egui_ctx);
        
        // Initialize ORT
        let _ = ort::init().with_name("local_lens").commit();

        let session = ml::find_model_file("resnet50-v2-7.onnx")
            .and_then(|path| {
                Session::builder()
                    .ok()
                    .and_then(|b| b.commit_from_file(path).ok())
            })
            .map(|s| Arc::new(Mutex::new(s)));

        let labels = Arc::new(ml::load_labels().unwrap_or_default());
        let (sender, receiver) = mpsc::channel();
        let ocr = Arc::new(crate::ocr::OcrModel::new());

        Self {
            folder: String::new(),
            query: String::new(),
            rename_old: String::new(),
            rename_new: String::new(),
            results: Vec::new(),
            conn: db::setup_db().unwrap(),
            session,
            labels,
            ocr,
            receiver,
            sender,
            is_processing: false,
            progress: (0, 0),
            logs: Vec::new(),
            cancellation_token: Arc::new(AtomicBool::new(false)),
        }
    }
    
    fn open_file(&self, path: &std::path::Path) {
        if let Err(e) = open::that(path) {
            eprintln!("Failed to open file: {}", e);
        }
    }

    fn configure_styles(ctx: &Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        visuals.widgets.open.rounding = egui::Rounding::same(8.0);
        visuals.window_rounding = egui::Rounding::same(12.0);
        visuals.selection.bg_fill = egui::Color32::from_rgb(100, 149, 237); // Cornflower Blue
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.spacing.window_margin = egui::Margin::same(10.0);
        ctx.set_style(style);
    }

    fn start_tagging(&mut self) {
        if self.is_processing {
            return;
        }
        
        // Basic validation
        if self.folder.is_empty() {
            self.logs.push("Please select a folder first.".to_string());
            return;
        }

        if let Some(session) = &self.session {
            self.is_processing = true;
            self.progress = (0, 0);
            self.logs.push("Starting tagging process...".to_string());
            
            // Reset token
            self.cancellation_token.store(false, Ordering::Relaxed);
            let token = self.cancellation_token.clone();

            let session = session.clone();
            let labels = self.labels.clone();
            let folder = self.folder.clone();
            let sender = self.sender.clone();
            let ocr = self.ocr.clone();

            thread::spawn(move || {
                processing::run_tagging_process(folder, session, labels, ocr, sender, token);
            });
        } else {
            self.logs.push("Model session not initialized.".to_string());
        }
    }
}

impl App for ImageTagger {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Handle messages
        while let Ok(msg) = self.receiver.try_recv() {
            match msg {
                AppMessage::Log(s) => self.logs.push(s),
                AppMessage::Progress(curr, total) => self.progress = (curr, total),
                AppMessage::Finished => self.is_processing = false,
            }
        }
        
        // --- Sidebar (Controls) ---
        SidePanel::left("control_panel").resizable(true).default_width(250.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("Local Lens");
            ui.separator();
            
            ui.group(|ui| {
                ui.heading("Tagging");
                ui.label("Folder path:");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.folder);
                    if ui.button("Browse...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.folder = path.display().to_string();
                        }
                    }
                });
                ui.add_space(5.0);
                if self.is_processing {
                    if ui.button("Cancel").clicked() {
                        self.cancellation_token.store(true, Ordering::Relaxed);
                        self.logs.push("Cancelling...".to_string());
                    }
                } else {
                     if ui.button("Tag Images").clicked() {
                        self.start_tagging();
                    }
                }
                
                if self.is_processing || self.progress.1 > 0 {
                    ui.add_space(10.0);

                    let progress = if self.progress.1 > 0 {
                        self.progress.0 as f32 / self.progress.1 as f32
                    } else {
                        0.0
                    };
                    ui.add(ProgressBar::new(progress).text(format!("{}/{}", self.progress.0, self.progress.1)));
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("Manage Tags");
                ui.horizontal(|ui| {
                    ui.label("Old:");
                    ui.text_edit_singleline(&mut self.rename_old);
                });
                ui.horizontal(|ui| {
                    ui.label("New:");
                    ui.text_edit_singleline(&mut self.rename_new);
                });
                
                if ui.button("Rename Globally").clicked() {
                     if !self.rename_old.is_empty() && !self.rename_new.is_empty() && !self.is_processing {
                        self.is_processing = true;
                        self.progress = (0, 0);
                        self.logs.push(format!("Renaming '{}' to '{}'...", self.rename_old, self.rename_new));
                        
                        let sender = self.sender.clone();
                        let old_tag = self.rename_old.clone();
                        let new_tag = self.rename_new.clone();
                        
                        thread::spawn(move || {
                            processing::run_renaming_process(old_tag, new_tag, sender);
                        });
                     }
                }
            });

            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("Search");
                ui.label("Search query:");
                ui.text_edit_singleline(&mut self.query);
                ui.add_space(5.0);
                if ui.button("Search").clicked() {
                     self.results = db::search_images(&self.conn, &self.query).unwrap_or_default();
                }
            });
        });

        // --- Bottom Panel (Logs) ---
        TopBottomPanel::bottom("log_panel").resizable(true).min_height(100.0).show(ctx, |ui| {
            ui.heading("Logs");
            ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                for log in &self.logs {
                    ui.label(log);
                }
            });
        });

        // --- Central Panel (Results) ---
        CentralPanel::default().show(ctx, |ui| {
            if self.results.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("No results found. Start by tagging a folder or searching.");
                });
            } else {
                ui.heading(format!("Results ({})", self.results.len()));
                ui.separator();
                
                ScrollArea::vertical().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for path in &self.results {
                            ui.allocate_ui(Vec2::new(160.0, 200.0), |ui| {
                                ui.vertical_centered(|ui| {
                                     let uri = format!("file://{}", path.display());
                                     // Interactive image
                                     let img_resp = ui.add(egui::Image::from_uri(uri).fit_to_exact_size(Vec2::splat(150.0)).sense(egui::Sense::click()));
                                     
                                     if img_resp.clicked() {
                                         self.open_file(path);
                                     }
                                     if img_resp.hovered() {
                                         ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                                     }
                                     
                                     ui.label(path.file_name().unwrap_or_default().to_string_lossy());
                                });
                            });
                        }
                    });
                });
            }
        });

        // Repaint if processing to show progress smoothly
        if self.is_processing {
            ctx.request_repaint();
        }
    }
}
