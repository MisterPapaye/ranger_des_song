use tauri::State;
use std::sync::Mutex;
use std::path::PathBuf;
use std::time::Instant;
use std::collections::HashSet; // Pour gérer les doublons en mémoire
use anyhow::Result;
use tauri::Emitter;

use crate::audio::{AudioMetadataExtractor, AudioFileProcessor};
use crate::models::{OrganizationResult, ProgressUpdate};
use crate::api::MusicBrainzClient;

pub struct AppState {
    pub source_folder: Mutex<Option<PathBuf>>,
    pub destination_folder: Mutex<Option<PathBuf>>,
}

#[tauri::command]
pub fn select_source_folder(_state: State<AppState>) -> String {
    "Source folder selected".to_string()
}

#[tauri::command]
pub fn select_destination_folder(_state: State<AppState>) -> String {
    "Destination folder selected".to_string()
}

#[tauri::command]
pub async fn start_organization(
    source: String,
    destination: String,
    window: tauri::Window,
) -> Result<OrganizationResult, String> {
    let start = Instant::now();
    let source_path = PathBuf::from(source);
    let dest_path = PathBuf::from(destination);

    if !source_path.exists() || !dest_path.exists() {
        return Err("Folders do not exist".to_string());
    }

    let audio_files = AudioFileProcessor::find_audio_files(&source_path)
        .map_err(|e| e.to_string())?;

    let total_files = audio_files.len();
    let mut organized_files = 0;
    let mut failed_files = 0;
    let mut duplicate_files = 0;
    
    // Set pour suivre les empreintes numériques (doublons de contenu)
    let mut seen_hashes = HashSet::new();
    let client = MusicBrainzClient::new();

    for (idx, audio_file) in audio_files.iter().enumerate() {
        let file_name = audio_file
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let _ = window.emit("progress", ProgressUpdate {
            processed: idx,
            total: total_files,
            current_file: file_name.to_string(),
            status: "Checking for duplicates...".to_string(),
        });

        // --- 1. Gestion des doublons par Hash ---
        let file_hash = match AudioFileProcessor::calculate_hash(audio_file) {
            Ok(h) => h,
            Err(_) => {
                failed_files += 1;
                continue;
            }
        };

        if seen_hashes.contains(&file_hash) {
            duplicate_files += 1;
            continue; // On saute ce fichier car il est déjà traité
        }
        seen_hashes.insert(file_hash);

        // --- 2. Extraction & MusicBrainz ---
        match AudioMetadataExtractor::extract(audio_file) {
            Ok(metadata) => {
                let (title, artist, genre) = if !metadata.has_complete_tags || metadata.genre == "Unknown" {
                    match client.search_recording(&metadata.title, &metadata.artist).await {
                        Ok(Some((mb_t, mb_a, mb_g))) => {
                            let final_genre = if mb_g != "Unknown" { mb_g } else { metadata.genre };
                            (mb_t, mb_a, final_genre)
                        },
                        _ => (metadata.title, metadata.artist, metadata.genre),
                    }
                } else {
                    (metadata.title, metadata.artist, metadata.genre)
                };

                let new_filename = AudioFileProcessor::sanitize_filename(&title, &artist, &genre);

                // --- 3. Copie & Vérification existence destination ---
                match AudioFileProcessor::copy_and_rename(
                    audio_file,
                    &dest_path,
                    &genre,
                    &new_filename,
                ) {
                    Ok(_) => organized_files += 1,
                    Err(e) => {
                        // Si l'erreur est que le fichier existe déjà, on compte comme doublon
                        if e.to_string().contains("exists") {
                            duplicate_files += 1;
                        } else {
                            failed_files += 1;
                        }
                    }
                }
            }
            Err(_) => failed_files += 1,
        }
    }

    Ok(OrganizationResult {
        total_files,
        organized_files,
        failed_files: failed_files + duplicate_files, // Tu peux aussi créer un champ spécifique pour les doublons
        duration_ms: start.elapsed().as_millis(),
    })
}