//! Stat cards and details table dispatcher.
//!
//! **Taxonomy Classification**: UI Rendering (UI Dispatcher).

pub mod cards;
pub mod details;
pub mod processes;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};
use crate::app::{App, FocusedSection};

pub fn render(f: &mut Frame, app: &mut App, chunks: std::rc::Rc<[Rect]>) {
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(chunks[1]);

    let card_w = top_chunks[0].width.saturating_sub(4);
    let card_w_usize = card_w as usize;

    let cpu_border = cards::border_color(app.focus, FocusedSection::Cpu, app.theme.accent, app.theme.border);
    let mem_border = cards::border_color(app.focus, FocusedSection::Memory, app.theme.accent, app.theme.border);
    let disk_border = cards::border_color(app.focus, FocusedSection::Disk, app.theme.accent, app.theme.border);
    let gpu_border = cards::border_color(app.focus, FocusedSection::Gpu, app.theme.accent, app.theme.border);
    let net_border = cards::border_color(app.focus, FocusedSection::Network, app.theme.accent, app.theme.border);

    cards::render_cpu_card(f, top_chunks[0], app, card_w, cpu_border);
    cards::render_memory_card(f, top_chunks[1], app, card_w, mem_border);
    cards::render_disk_card(f, top_chunks[2], app, card_w, disk_border);
    cards::render_gpu_card(f, top_chunks[3], app, card_w, card_w_usize, gpu_border);
    cards::render_network_card(f, top_chunks[4], app, card_w, net_border);

    render_context_table(f, chunks[2], app);
}

fn render_context_table(f: &mut Frame, area: Rect, app: &mut App) {
    let theme = &*app.theme;
    let border_color = theme.accent;

    let details_height: u16 = match app.focus {
        FocusedSection::Cpu => {
            let num_cpus = app.sys.cpus().len();
            let inner = area.width as usize;
            let cols = (inner / 10).max(1);
            let rows = (num_cpus + cols - 1) / cols;
            (rows as u16 + 2).clamp(4, 15)
        }
        FocusedSection::Memory => 11,
        FocusedSection::Disk => (app.disks.len() as u16 + 4).clamp(6, 15),
        FocusedSection::Gpu => (app.gpu_names.len() as u16 + 4).clamp(6, 12),
        FocusedSection::Network => (app.networks.len() as u16 + 4).clamp(6, 15),
    };

    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(details_height), Constraint::Min(4)])
        .split(area);

    match app.focus {
        FocusedSection::Cpu => details::render_cpu_details(f, sub_chunks[0], app, border_color),
        FocusedSection::Memory => details::render_memory_details(f, sub_chunks[0], app, border_color),
        FocusedSection::Disk => details::render_disk_details(f, sub_chunks[0], app, border_color),
        FocusedSection::Gpu => details::render_gpu_details(f, sub_chunks[0], app, border_color),
        FocusedSection::Network => details::render_network_details(f, sub_chunks[0], app, border_color),
    }

    processes::render_processes_table(f, sub_chunks[1], app);
}
