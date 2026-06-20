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

pub mod app;
pub mod event;
pub mod process;
pub mod procfile;
pub mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let procfile_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Procfile".to_string());
    let terminal = ratatui::init();
    let result = app::App::new(procfile_path).run(terminal).await;
    ratatui::restore();
    result
}
