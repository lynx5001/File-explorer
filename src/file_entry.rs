use std::fs;
use std::path::PathBuf;

pub struct FileEntry {
    pub name: String,     // name of file or folder
    pub path: String,     // whole path to the entry
    pub is_dir: bool,     // true for folder, false for file
    pub size: Option<u64>, // size of files in Bytes, Option<u64> due to folders having no size
}

impl FileEntry {
    // constructor for FileEntry
    pub fn new(name: String, path: String, is_dir: bool, size: Option<u64>) -> Self {
        Self {
            name,
            path,
            is_dir,
            size,
        }
    }

    // function to initialize the struct values of an entry
    pub fn initialize_entry(entry: fs::DirEntry) -> Option<Self> {
        // get metadata
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(_) => return None,
        };

        // initialize FileEntry values
        let path = entry.path();
        let name = path.file_name()?.to_string_lossy().into_owned();
        let is_dir = metadata.is_dir();
        let size = if metadata.is_file() {
            Some(metadata.len())
        } else {
            None
        };

        // return initialized FileEntry 
        Some(FileEntry::new(name, path.to_string_lossy().into_owned(), is_dir, size))
    }

    // function to determine size of a file
    pub fn determine_size(&self) -> String {
        if self.is_dir {
            // folders have no size
            return String::from("--")
        }

        // match size
        if let Some(size) = self.size {
            const KB: u64 = 1024;
            const MB: u64 = KB * 1024;
            const GB: u64 = MB * 1024;

            if size >= GB {
                format!("{:.2} GB", size as f64 / GB as f64)
            } else if size >= MB {
                format!("{:.2} MB", size as f64 / MB as f64)
            } else if size >= KB {
                format!("{:.2} KB", size as f64 / KB as f64)
            } else {
                format!("{} Bytes", size)
            }
        } else {
            // for unknown size
            String::from("N/A")
        }
    }

    // function to determine icon of a file, based on its type
    pub fn determine_icon(&self) -> &'static str {
        // default icon
        if self.is_dir {
            return "📁 ";
        }

        // convert string to pathbuf to use .extension()
        let path_buf = std::path::PathBuf::from(&self.path);

        // get file ending 
        if let Some(ext) = path_buf.extension() {
            // convert to string and match case insensitiv for ending
            if let Some(ext_str) = ext.to_str() {
                match ext_str.to_lowercase().as_str() {
                    "txt" | "log" | "md" => "📄 ", // text-file
                    "pdf" => "📄 ", // pdf-file
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" => "📷 ", // image-file
                    "mp3" | "wav" | "flac" | "ogg" => "🎵 ", // audio-file
                    "mp4" | "avi" | "mkv" | "mov" => "📷 ", // video-file
                    "zip" | "rar" | "7z" | "tar" | "gz" => "📦 ", // archiv
                    "exe" | "app" | "bin" => "⚙️ ", // executable
                    "doc" | "docx" | "odt" => "📄 ", // word-file
                    "xls" | "xlsx" | "ods" => "📄 ", // tabular-file
                    "ppt" | "pptx" | "odp" => "📄 ", // slides
                    "json" | "xml" | "yaml" | "yml" => "⚙️ ", // config-file
                    "html" | "htm" | "css" | "js" | "php" | "py" | "rs" | "c" | "cpp" | "h" | "java" => "⚙️ ", // code-file
                    _ => "❓ ", // all other cases
                }
            } else {
                // if ending is not utf8
                "❓ "
            }
        } else {
            // files without ending
            "❓ "
        }
    }
}

// function to display files and dirs
pub fn read_directory_contents(directory: &PathBuf) -> Result<Vec<FileEntry>, String> {
    // define entries variable to store current paths and files
    let mut entries = Vec::new();

    let dir_iter = match fs::read_dir(directory) {
        Ok(iter) => iter,
        Err(e) => {
            return Err(format!("Verzeichnis '{}' konnte nicht gelesen werden: {}", directory.display(), e));
        }
    };

    for entry_result in dir_iter {
        match entry_result {
            Ok(dir_entry) => {
                let entry_path_for_error_msg = dir_entry.path();

                // Konvertiere fs::DirEntry zu deiner FileEntry
                if let Some(file_entry) = FileEntry::initialize_entry(dir_entry) {
                    entries.push(file_entry);
                } else {
                    // Hier kannst du protokollieren, wenn initialize_entry fehlschlägt
                    // (z.B. weil Metadaten nicht gelesen werden konnten)
                    eprintln!("Warnung: Konnte Eintrag '{}' nicht verarbeiten (Metadaten-Fehler o.ä.).", entry_path_for_error_msg.display());
                }
            }
            Err(e) => {
                // Hier wird ein Fehler beim Lesen eines *einzelnen* Eintrags behandelt
                // Du kannst ihn protokollieren und fortfahren, oder die ganze Operation abbrechen.
                // Für einen File Explorer ist "protokollieren und fortfahren" oft sinnvoll.
                eprintln!("Fehler beim Lesen eines Verzeichniseintrags: {}", e);
            }
        }
    }

    // Sort entries: directories first, then alphabetically by name
    entries.sort_by(|a: &FileEntry, b: &FileEntry| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    //return entries
    Ok(entries)
}

// Function to handle user input for navigation
pub fn handle_user_input(input: &str, p_current_directory: &mut PathBuf){
    if input.trim() == "up"{
        // Move to the parent directory
        if let Some(parent) = p_current_directory.parent() {
            *p_current_directory = parent.to_path_buf();
        } else {
            println!("Already at the root directory.");
        }
    }
    else {
        // Move into the selected directory
        p_current_directory.push(input.trim());
    }
}
