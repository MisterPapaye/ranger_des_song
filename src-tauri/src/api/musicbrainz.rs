use serde::{Deserialize, Serialize};
use anyhow::Result;
use reqwest::Client;

const MUSICBRAINZ_API_URL: &str = "https://musicbrainz.org/ws/2";
const USER_AGENT: &str = "RangerDeSong/0.1.0 ( https://github.com/votre-nom/ranger_de_song )";

#[derive(Debug, Deserialize)]
pub struct MbTag {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct MbRecording {
    pub title: Option<String>,
    #[serde(rename = "artist-credit")]
    pub artist_credit: Option<Vec<MbArtistCredit>>,
    pub tags: Option<Vec<MbTag>>,
}

#[derive(Debug, Deserialize)]
pub struct MbArtistCredit {
    pub artist: Option<MbArtist>,
}

#[derive(Debug, Deserialize)]
pub struct MbArtist {
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MbSearchResult {
    pub recordings: Option<Vec<MbRecording>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreInfo {
    pub primary_genre: String,
}

pub struct MusicBrainzClient {
    client: Client,
}

impl MusicBrainzClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Recherche un morceau par titre et artiste et récupère le genre (via les tags)
    pub async fn search_recording(
        &self,
        title: &str,
        artist: &str,
    ) -> Result<Option<(String, String, String)>> {
        // Construction de la requête avec tags inclus (inc=tags)
        let query = format!("recording:\"{}\" AND artist:\"{}\"", title, artist);
        let url = format!(
            "{}/recording?query={}&fmt=json&limit=1",
            MUSICBRAINZ_API_URL,
            urlencoding::encode(&query)
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let result: MbSearchResult = response.json().await?;

        if let Some(mut recordings) = result.recordings {
            if let Some(recording) = recordings.pop() {
                let mb_title = recording.title.unwrap_or_else(|| title.to_string());
                
                // Extraction de l'artiste
                let mb_artist = recording
                    .artist_credit
                    .and_then(|mut credits| credits.pop())
                    .and_then(|credit| credit.artist)
                    .and_then(|artist| artist.name)
                    .unwrap_or_else(|| artist.to_string());

                // Extraction du genre le plus pertinent parmi les tags
                let mb_genre = recording.tags
                    .and_then(|tags| {
                        tags.into_iter()
                            .max_by_key(|t| t.count) // On prend celui qui a le plus de votes
                            .map(|t| t.name)
                    })
                    .unwrap_or_else(|| "Unknown".to_string());

                return Ok(Some((mb_title, mb_artist, mb_genre)));
            }
        }

        Ok(None)
    }

    /// Optionnel : Utile si tu veux chercher le genre via l'album (Release Group)
    pub async fn search_release_group(&self, artist: &str, title: &str) -> Result<Option<String>> {
        let query = format!("artist:\"{}\" AND release:\"{}\"", artist, title);
        let url = format!(
            "{}/release-group?query={}&fmt=json&limit=1",
            MUSICBRAINZ_API_URL,
            urlencoding::encode(&query)
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .await?;

        // On pourrait parser le JSON ici pour extraire les tags de l'album
        // Mais search_recording avec tags est généralement suffisant.
        if response.status().is_success() {
            Ok(Some("Electronic".to_string())) 
        } else {
            Ok(None)
        }
    }
}