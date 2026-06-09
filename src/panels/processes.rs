//! Processes list rendering table.
//!
//! **Taxonomy Classification**: UI Rendering (Process Table).

use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};
use crate::app::{App, FocusedSection, ProcessItem};
use crate::helpers::format_speed;

pub fn render_processes_table(f: &mut Frame, area: Rect, app: &mut App) {
    let theme = &*app.theme;
    let rows: Vec<Row> = app
        .processes
        .iter()
        .map(|p: &ProcessItem| {
            let pid = p.pid.to_string();
            let name = p.name.clone();
            let cpu = format!("{:.1}%", p.cpu);
            let mem_mb = p.mem as f64 / 1024.0 / 1024.0;
            let mem = format!("{:.1} MB", mem_mb);
            let disk_speed = (p.disk_read + p.disk_write) as f64 / 1.5;
            let storage = if disk_speed > 0.0 {
                format_speed(disk_speed)
            } else {
                "0 B/s".to_string()
            };
            let gpu = format!("{:.1}%", p.gpu);
            let net_speed = p.net as f64;
            let net = if net_speed > 0.0 {
                format_speed(net_speed)
            } else {
                "0 B/s".to_string()
            };
            let focused = app.focus;
            let cpu_style = if focused == FocusedSection::Cpu {
                Style::default().fg(Color::Rgb(80, 250, 123)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_main)
            };
            let mem_style = if focused == FocusedSection::Memory {
                Style::default().fg(Color::Rgb(255, 121, 198)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_main)
            };
            let storage_style = if focused == FocusedSection::Disk {
                Style::default().fg(Color::Rgb(160, 32, 240)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_main)
            };
            let gpu_style = if focused == FocusedSection::Gpu {
                Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_main)
            };
            let net_style = if focused == FocusedSection::Network {
                Style::default().fg(Color::Rgb(0, 245, 255)).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_main)
            };
            Row::new(vec![
                Cell::from(pid).style(Style::default().fg(theme.text_dim)),
                Cell::from(name).style(
                    Style::default().fg(theme.text_main).add_modifier(Modifier::BOLD),
                ),
                Cell::from(cpu).style(cpu_style),
                Cell::from(mem).style(mem_style),
                Cell::from(storage).style(storage_style),
                Cell::from(gpu).style(gpu_style),
                Cell::from(net).style(net_style),
            ])
        })
        .collect();
    let widths = [
        Constraint::Length(8),
        Constraint::Percentage(24),
        Constraint::Percentage(13),
        Constraint::Percentage(14),
        Constraint::Percentage(15),
        Constraint::Percentage(12),
        Constraint::Percentage(14),
    ];
    let header_color = |active: bool, color: Color| {
        if active {
            Style::default().fg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)
        }
    };
    let headers = vec![
        Cell::from("PID").style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
        Cell::from("Process Name").style(
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ),
        Cell::from("CPU").style(header_color(app.focus == FocusedSection::Cpu, Color::Rgb(80, 250, 123))),
        Cell::from("Memory").style(header_color(app.focus == FocusedSection::Memory, Color::Rgb(255, 121, 198))),
        Cell::from("Storage").style(header_color(app.focus == FocusedSection::Disk, Color::Rgb(160, 32, 240))),
        Cell::from("GPU").style(header_color(app.focus == FocusedSection::Gpu, Color::Rgb(255, 215, 0))),
        Cell::from("Network").style(header_color(app.focus == FocusedSection::Network, Color::Rgb(0, 245, 255))),
    ];
    let table_title = match app.focus {
        FocusedSection::Cpu => " Active Processes (Sorted by CPU) ",
        FocusedSection::Memory => " Active Processes (Sorted by RAM) ",
        FocusedSection::Disk => " Active Processes (Sorted by Disk I/O) ",
        FocusedSection::Gpu => " Active Processes (Sorted by GPU) ",
        FocusedSection::Network => " Active Processes (Sorted by Network) ",
    };
    let process_border_color = theme.accent;
    let table = Table::new(rows, widths)
        .header(
            Row::new(headers)
                .style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(table_title)
                .title_style(
                    Style::default().fg(Color::Rgb(255, 215, 0)).add_modifier(Modifier::BOLD),
                )
                .border_style(Style::default().fg(process_border_color)),
        )
        .row_highlight_style(
            Style::default()
                .bg(app.theme.highlight_bg)
                .fg(Color::Rgb(248, 248, 242))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(table, area, &mut app.process_state);
}
