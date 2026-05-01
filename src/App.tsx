import "./App.css";
import React, { useState, useRef } from "react";
// Nouveaux imports Tauri v2
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { open as fileOpen } from "@tauri-apps/plugin-dialog";

interface ProgressUpdate {
  processed: number;
  total: number;
  current_file: string;
  status: string;
}

interface OrganizationResult {
  total_files: number;
  organized_files: number;
  failed_files: number;
  duration_ms: number;
}

function App() {
  const [sourceFolder, setSourceFolder] = useState<string>("");
  const [destFolder, setDestFolder] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState<ProgressUpdate | null>(null);
  const [result, setResult] = useState<OrganizationResult | null>(null);
  const [error, setError] = useState<string>("");

  // Mise à jour du type de la ref pour l'unlisten de Tauri v2
  const unlistenRef = useRef<UnlistenFn | null>(null);

  const selectSourceFolder = async () => {
    try {
      const selected = await fileOpen({
        directory: true,
        multiple: false,
        title: "Select Source Music Folder",
      });
      // En v2, selected peut être null si l'utilisateur annule
      if (selected) {
        setSourceFolder(selected as string);
        setError("");
      }
    } catch (err) {
      setError(`Failed to open folder picker: ${err}`);
    }
  };

  const selectDestFolder = async () => {
    try {
      const selected = await fileOpen({
        directory: true,
        multiple: false,
        title: "Select Destination Folder",
      });
      if (selected) {
        setDestFolder(selected as string);
        setError("");
      }
    } catch (err) {
      setError(`Failed to open folder picker: ${err}`);
    }
  };

  const startOrganization = async () => {
    if (!sourceFolder || !destFolder) {
      setError("Please select both source and destination folders");
      return;
    }

    setLoading(true);
    setProgress(null);
    setResult(null);
    setError("");

    try {
      // Ecoute des progrès version v2
      // Note: 'progress' doit correspondre au nom d'événement envoyé par Emitter dans Rust
      const unlisten = await listen<ProgressUpdate>("progress", (event) => {
        setProgress(event.payload);
      });

      unlistenRef.current = unlisten;

      const organizationResult: OrganizationResult = await invoke(
        "start_organization",
        {
          source: sourceFolder,
          destination: destFolder,
        }
      );

      setResult(organizationResult);
    } catch (err) {
      setError(`Error: ${err}`);
    } finally {
      if (unlistenRef.current) {
        unlistenRef.current();
        unlistenRef.current = null;
      }
      setLoading(false);
    }
  };

  const progressPercent =
    progress && progress.total > 0
      ? Math.round((progress.processed / progress.total) * 100)
      : 0;

  return (
    <div className="app">
      {/* Le reste du JSX reste identique à ton code original */}
      <header className="header">
        <h1>🎵 Ranger de Song</h1>
        <p className="subtitle">DJ Music Library Organizer</p>
      </header>

      <main className="main">
        <section className="folder-selection">
          <div className="folder-group">
            <label>Source Music Folder</label>
            <div className="input-group">
              <input
                type="text"
                placeholder="Select your music folder..."
                value={sourceFolder}
                readOnly
                className="folder-path"
              />
              <button onClick={selectSourceFolder} disabled={loading}>
                Browse
              </button>
            </div>
          </div>

          <div className="folder-group">
            <label>Destination Folder</label>
            <div className="input-group">
              <input
                type="text"
                placeholder="Select destination folder..."
                value={destFolder}
                readOnly
                className="folder-path"
              />
              <button onClick={selectDestFolder} disabled={loading}>
                Browse
              </button>
            </div>
          </div>
        </section>

        {error && <div className="error-message">{error}</div>}

        {progress && (
          <section className="progress-section">
            <div className="progress-info">
              <p className="current-file">📁 {progress.current_file}</p>
              <p className="status">{progress.status}</p>
              <p className="count">
                {progress.processed} / {progress.total} files
              </p>
            </div>

            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${progressPercent}%` }}
              ></div>
            </div>
            <p className="progress-percent">{progressPercent}%</p>
          </section>
        )}

        {result && (
          <section className="results-section">
            <h2>Organization Complete! ✅</h2>
            <div className="results-grid">
              <div className="result-card">
                <p className="result-label">Total Files</p>
                <p className="result-value">{result.total_files}</p>
              </div>
              <div className="result-card success">
                <p className="result-label">Organized</p>
                <p className="result-value">{result.organized_files}</p>
              </div>
              <div className="result-card warning">
                <p className="result-label">Failed</p>
                <p className="result-value">{result.failed_files}</p>
              </div>
              <div className="result-card info">
                <p className="result-label">Duration</p>
                <p className="result-value">{(result.duration_ms / 1000).toFixed(2)}s</p>
              </div>
            </div>
          </section>
        )}

        <section className="action-section">
          <button
            className="start-button"
            onClick={startOrganization}
            disabled={loading || !sourceFolder || !destFolder}
          >
            {loading ? "Organizing..." : "Start Organization"}
          </button>
        </section>
      </main>

      <footer className="footer">
        <p>Ranger de Song v0.1.0 - Organize your music library with style</p>
      </footer>
    </div>
  );
}

export default App;