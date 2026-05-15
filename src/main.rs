mod file_entry;

use file_entry::FileEntry;
use std::path::PathBuf;
use eframe::{egui, App, Frame};
use file_entry::{handle_user_input, read_directory};

fn main() -> eframe::Result<()> {
    // default options for the native window
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Rust File Explorer",
        native_options,
        Box::new(|cc| Ok(Box::new(FileExplorerApp::new(cc))))    
        )
}

// compiler automatically creates default function, 
// which sets default values for the following data structure
#[derive(Default)]
// struct has all variable which interact with the ui
struct FileExplorerApp {
    current_path: PathBuf,
    entries: Vec<FileEntry>,
    input_text: String,
    last_error: Option<String>,
    clipboard: Option<String>,
    pending_delete: Option<String>,
    needs_reload: bool,
}

impl FileExplorerApp {
    //app construtor
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            // initialize app variables
            current_path: PathBuf::from("/Users/bp/"),
            entries: Vec::new(),
            input_text: "".to_string(),
            last_error: None,
            clipboard: None,
            pending_delete: None,
            needs_reload: false,
        };
        // load current content
        app.load_content();
        //return app
        app
    }

    // load subfolders and files from current directory
    fn load_content(&mut self) {
        // clear errors and entries
        self.entries.clear();
        self.last_error = None;

        // call read dir function from file_entry
        match read_directory(&self.current_path) {
            Ok(loaded_entries) => {
                self.entries = loaded_entries;
                self.input_text = self.current_path.display().to_string();
            }
            Err(e) => {
                eprintln!("Error loading directory contents for {}: {}", self.current_path.display(), e);
                //TODO: display error in UI later
                self.last_error = Some(e);
                self.input_text = self.current_path.display().to_string();
            }
        }
    }
}

impl App for FileExplorerApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        // top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // move back button
                let button = ui.button("Back");
                if button.clicked() {
                    handle_user_input("up", &mut self.current_path);
                    self.load_content();
                }

                // configure search text
                let is_default_text = self.input_text == self.current_path.display().to_string();
                let mut text_edit_widget = egui::TextEdit::singleline(&mut self.input_text).desired_width(f32::INFINITY);;
                if is_default_text {
                    text_edit_widget = text_edit_widget.text_color(egui::Color32::from_rgb(150, 150, 150));
                }

                // add search bar with default text
                let response = ui.add(text_edit_widget);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    handle_user_input(&self.input_text, &mut self.current_path);
                    self.load_content();
                    // self.input_text.clear();
                }
            });

            // display for current path
            ui.label(format!("Current Path: {}", self.current_path.display()));
            // last_error anzeigen
            // if let Some(err_msg) = &self.last_error {
            //     // Zeigt die Fehlermeldung in Rot an
            //     ui.label(
            //         egui::RichText::new(format!("Fehler: {}", err_msg))
            //             .color(egui::Color32::RED) 
            //     );
            //     // // button zum schließen der
            //     // if ui.button("X").clicked() {
            //     //     self.last_error = None;
            //     // }
            // }
        });

        // left panel
        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(350.0)
            .width_range(100.0..=500.0)
            .show(ctx, |ui| {

                let mut new_path_to_navigate_to: Option<PathBuf> = None;
                let mut file_to_open_path: Option<PathBuf> = None;

                // start scroll area
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("file_explorer_grid") // ID für Grid
                        .num_columns(2)
                        .spacing(egui::vec2(20.0, 4.0))
                        .show(ui, |ui| { //Grid inhalt
                            ui.strong("Name");
                            ui.strong("Size");
                            ui.end_row(); // end row and start new

                            // list current directories and paths
                            for entry in &self.entries {
                                // Simple display: icon + name
                                let icon = entry.determine_icon();
                                let response_labels = ui.selectable_label(false, format!("{}{}", icon, entry.name));       
                                ui.label(entry.determine_size());
                                ui.end_row();

                                response_labels.context_menu(|ui| {
                                    if ui.button("Kopieren").clicked() {
                                        file_entry::copy_to_clipboard(&entry.path, &mut self.clipboard);
                                        ui.close_menu();
                                    }
                                    if let Some(path) = self.pending_delete.clone() {
                                        egui::Window::new("Bestätigung")
                                            .collapsible(false)
                                            .resizable(false)
                                            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                                            .show(ctx, |ui| {
                                                ui.label(format!("Wirklich löschen?\n{}", path));
                                                ui.horizontal(|ui| {
                                                    if ui.button("Löschen").clicked() {
                                                        if let Err(e) = file_entry::delete_entry(&path) {
                                                            self.last_error = Some(e);
                                                        } else {
                                                            self.needs_reload = true;
                                                        }
                                                        self.pending_delete = None;
                                                    }
                                                    if ui.button("Abbrechen").clicked() {
                                                        self.pending_delete = None;
                                                    }
                                                });
                                            });
                                    }
                                });

                                if response_labels.clicked() {
                                    if entry.is_dir {
                                        // Pfad für Navigation speichern, nicht direkt ändern
                                        new_path_to_navigate_to = Some(self.current_path.join(&entry.name));
                                    } else {
                                        // Pfad für Datei öffnen speichern
                                        file_to_open_path = Some(PathBuf::from(&entry.path));
                                    }
                                }
                            }
                        })

                    

                });

                // Aktionen ausführen, nachdem der Loop und damit der immutable borrow beendet ist
                if let Some(path) = new_path_to_navigate_to {
                    self.current_path = path;
                    self.load_content(); // Mutable borrow von self ist hier erlaubt
                }

                if let Some(path) = file_to_open_path {
                    // Plattformspezifisches Öffnen der Datei
                    #[cfg(target_os = "windows")]
                    {
                        std::process::Command::new("cmd").arg("/C").arg("start").arg(&path).spawn().ok();
                    }
                    #[cfg(target_os = "macos")]
                    {
                        std::process::Command::new("open").arg(&path).spawn().ok();
                    }
                    #[cfg(target_os = "linux")]
                    {
                        std::process::Command::new("xdg-open").arg(&path).spawn().ok();
                    }
                    // TODO: Fehlerbehandlung oder Feedback für den Benutzer
                }

                if self.needs_reload {
                    self.needs_reload = false;
                    self.load_content();
                }
        });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}


// // implement functions for app struct
// impl FileExplorerApp {
//     // app construtor
//     fn new(_cc: &eframe::CreationContext<'_>) -> Self {
//         let mut app = Self {
//             // initialize app variables
//             current_path: PathBuf::from("/Users/bp/"),
//             entries: Vec::new(),
//             input_text: "".to_string(),
//             last_error: None,
//             clipboard: None,
//             pending_delete: None,
//             needs_reload: false,
//         };
//         // load current content
//         app.load_content();
//         //return app
//         app
//     }

//     // load subfolders and files from current directory
//     fn load_content(&mut self) {
//         // clear errors and entries
//         self.entries.clear();
//         self.last_error = None;

//         // call read dir function from file_entry
//         match read_directory(&self.current_path) {
//             Ok(loaded_entries) => {
//                 self.entries = loaded_entries;
//                 self.input_text = self.current_path.display().to_string();
//             }
//             Err(e) => {
//                 eprintln!("Error loading directory contents for {}: {}", self.current_path.display(), e);
//                 //TODO: display error in UI later
//                 self.last_error = Some(e);
//                 self.input_text = self.current_path.display().to_string();
//             }
//         }
//     }
// }

// impl App for FileExplorerApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {

//         egui::Panel::top("top_panel")
//             .resizable(true)
//             .min_size(32.0)
//             .show_inside(ctx, |ctx| {
//                 egui::ScrollArea::vertical().show(ctx, |ctx| {
//                     ctx.vertical_centered(|ctx| {
//                         ctx.heading("Expandable Upper Panel");
//                     });
//                     "laksjdflkasjdflkjs";
//                 });
//             });

//         // egui::CentralPanel::default().show(ctx, |ui| {
//         //     ui.horizontal(|ui| {
//         //         // move back button
//         //         let button = ui.button("<");
//         //         if button.clicked() {
//         //             handle_user_input("up", &mut self.current_path);
//         //             self.load_content();
//         //         }

//         //         if ui.button("Einfügen").clicked() {
//         //             if let Err(e) = file_entry::paste_from_clipboard(&self.clipboard, &self.current_path) {
//         //                 self.last_error = Some(e);
//         //             }
//         //             self.load_content();
//         //         }

//         //         let is_default_text = self.input_text == self.current_path.display().to_string();

//         //         let mut text_edit_widget = egui::TextEdit::singleline(&mut self.input_text);

//         //         if is_default_text {
//         //             text_edit_widget = text_edit_widget.text_color(egui::Color32::from_rgb(150, 150, 150)); // Ein mittleres Grau
//         //             // Oder ein helleres Grau, z.B. egui::Color32::LIGHT_GRAY
//         //             // Du könntest auch text_edit_widget.hint_text("Enter path or 'up'") hinzufügen,
//         //             // aber das würde nur angezeigt, wenn das Feld leer ist.
//         //         }

//         //         // Füge das konfigurierte Textfeld zum UI hinzu
//         //         let response = ui.add(text_edit_widget);


//         //         // search bar for paths
//         //         // let response = ui.text_edit_singleline(&mut self.input_text);
//         //         if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
//         //             handle_user_input(&self.input_text, &mut self.current_path);
//         //             self.load_content();
//         //             // self.input_text.clear();
//         //         }
//         //     });

//         //     // display for current path
//         //     // ui.label(format!("Current Path: {}", self.current_path.display()));
            
//         //     // separator element
//         //     ui.separator();

//         //     // HIER die last_error anzeigen:
//         //     if let Some(err_msg) = &self.last_error {
//         //         // Zeigt die Fehlermeldung in Rot an
//         //         ui.label(
//         //             egui::RichText::new(format!("Fehler: {}", err_msg))
//         //                 .color(egui::Color32::RED) 
//         //         );
//         //         // // button zum schließen der
//         //         // if ui.button("X").clicked() {
//         //         //     self.last_error = None;
//         //         // }
//         //     }

//         //     let mut new_path_to_navigate_to: Option<PathBuf> = None;
//         //     let mut file_to_open_path: Option<PathBuf> = None;

//         //     // start scroll area
//         //     egui::ScrollArea::vertical().show(ui, |ui| {
//         //         egui::Grid::new("file_explorer_grid") // ID für Grid
//         //             .num_columns(2)
//         //             .spacing(egui::vec2(20.0, 4.0))
//         //             .show(ui, |ui| { //Grid inhalt
//         //                 ui.strong("Name");
//         //                 ui.strong("Size");
//         //                 ui.end_row(); // end row and start new

//         //                 // list current directories and paths
//         //                 for entry in &self.entries {
//         //                     // Simple display: icon + name
//         //                     let icon = entry.determine_icon();
//         //                     let response_labels = ui.selectable_label(false, format!("{}{}", icon, entry.name));       
//         //                     ui.label(entry.determine_size());
//         //                     ui.end_row();

//         //                     response_labels.context_menu(|ui| {
//         //                         if ui.button("Kopieren").clicked() {
//         //                             file_entry::copy_to_clipboard(&entry.path, &mut self.clipboard);
//         //                             ui.close_menu();
//         //                         }
//         //                         if let Some(path) = self.pending_delete.clone() {
//         //                             egui::Window::new("Bestätigung")
//         //                                 .collapsible(false)
//         //                                 .resizable(false)
//         //                                 .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
//         //                                 .show(ctx, |ui| {
//         //                                     ui.label(format!("Wirklich löschen?\n{}", path));
//         //                                     ui.horizontal(|ui| {
//         //                                         if ui.button("Löschen").clicked() {
//         //                                             if let Err(e) = file_entry::delete_entry(&path) {
//         //                                                 self.last_error = Some(e);
//         //                                             } else {
//         //                                                 self.needs_reload = true;
//         //                                             }
//         //                                             self.pending_delete = None;
//         //                                         }
//         //                                         if ui.button("Abbrechen").clicked() {
//         //                                             self.pending_delete = None;
//         //                                         }
//         //                                     });
//         //                                 });
//         //                         }
//         //                     });

//         //                     if response_labels.clicked() {
//         //                         if entry.is_dir {
//         //                             // Pfad für Navigation speichern, nicht direkt ändern
//         //                             new_path_to_navigate_to = Some(self.current_path.join(&entry.name));
//         //                         } else {
//         //                             // Pfad für Datei öffnen speichern
//         //                             file_to_open_path = Some(PathBuf::from(&entry.path));
//         //                         }
//         //                     }
//         //                 }
//         //             })

                

//         //     });

//         //     // Aktionen ausführen, nachdem der Loop und damit der immutable borrow beendet ist
//         //     if let Some(path) = new_path_to_navigate_to {
//         //         self.current_path = path;
//         //         self.load_content(); // Mutable borrow von self ist hier erlaubt
//         //     }

//         //     if let Some(path) = file_to_open_path {
//         //         // Plattformspezifisches Öffnen der Datei
//         //         #[cfg(target_os = "windows")]
//         //         {
//         //             std::process::Command::new("cmd").arg("/C").arg("start").arg(&path).spawn().ok();
//         //         }
//         //         #[cfg(target_os = "macos")]
//         //         {
//         //             std::process::Command::new("open").arg(&path).spawn().ok();
//         //         }
//         //         #[cfg(target_os = "linux")]
//         //         {
//         //             std::process::Command::new("xdg-open").arg(&path).spawn().ok();
//         //         }
//         //         // TODO: Fehlerbehandlung oder Feedback für den Benutzer
//         //     }
//         // });

//         // if self.needs_reload {
//         //     self.needs_reload = false;
//         //     self.load_content();
//         // }
//     }
// }

