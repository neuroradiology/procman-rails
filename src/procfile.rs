//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ProcfileEntry {
    pub name: String,
    pub command: String,
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<ProcfileEntry>> {
    let content = std::fs::read_to_string(path).context("Failed to read Procfile")?;
    let mut entries = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((name, command)) = line.split_once(':') {
            entries.push(ProcfileEntry {
                name: name.trim().to_string(),
                command: command.trim().to_string(),
            });
        }
    }

    Ok(entries)
}
