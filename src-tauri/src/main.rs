// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // On appelle la fonction run du fichier lib.rs de ton propre projet
    ranger_de_song::run()
}