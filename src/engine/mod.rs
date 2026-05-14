use std::path::PathBuf;

use bevy::prelude::*;
use stockfish::Stockfish;

pub struct StockfishEngine {
    engine: Stockfish,
    is_thinking: bool,
    best_move: Option<String>
}

impl StockfishEngine {
    pub fn new() -> Result<Self, String> {
        let stockfish_path = Self::find_stockfish()
            .ok_or("coulnd't find stockfish on your system")?;

        let mut engine = Stockfish::new(stockfish_path)
            .map_err(|e| format!("failed to initilize stockfish: {e}"))?;

        engine.set_depth(8);

        Ok(Self {
            engine,
            is_thinking: false,
            best_move: None
        })
    }

    fn find_stockfish<'a>() -> Option<&'a str> {
        #[cfg(target_os = "windows")]
        {
            let paths = vec![
            "C:\\Program Files\\stockfish\\stockfish.exe",
            "C:\\Program Files (x86)\\stockfish\\stockfish.exe",
            "stockfish.exe", // Check if in PATH
        ];
            for path in paths {
                let pb = PathBuf::from(path);
                if pb.exists() {
                    return Some(path);
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let paths = vec![
                "/usr/local/bin/stockfish",
                "/opt/homebrew/bin/stockfish",
                "stockfish", // Check if in PATH
            ];
            for path in paths {
                let pb = PathBuf::from(path);
                if pb.exists() {
                    return Some(path)
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            let paths = vec![
                "/usr/bin/stockfish",
                "/usr/local/bin/stockfish",
                "stockfish", // Check if in PATH
            ];
            for path in paths {
                let pb = PathBuf::from(path);
                if pb.exists() {
                    return Some(path);
                }
            }
        }

        None
    }

    pub fn set_position(&mut self, fen: &str) {
        let _ = self.engine.set_fen_position(fen);
    }

    pub fn get_best_move(&mut self) -> Option<String> {
        match self.engine.go() {
            Ok(engine_output) => {
                let best_move = engine_output.best_move().to_string();
                self.best_move = Some(best_move.clone());
                Some(best_move)
            }
            Err(e) => {
                eprintln!("Stockfish error: {e}");
                None
            }
        }
    }

    pub fn set_is_thinking(&mut self, thinking: bool) {
        self.is_thinking = thinking;
    }
}


impl Default for StockfishEngine {
    fn default() -> Self {
        StockfishEngine::new()
            .expect("failed to create stockfishengine struct")
    }
}