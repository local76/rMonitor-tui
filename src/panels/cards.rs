//! Top-row stat cards (CPU, Memory, Disk, GPU, Network) rendering.
//!
//! **Taxonomy Classification**: UI Rendering (Stat Cards).

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, FocusedSection};
use crate::helpers::{draw_spring_bar, format_gpu_line, format_speed};

pub fn border_color(focus: FocusedSection, is: FocusedSection, accent: Color, border: Color) -> Color {
    if focus == is { accent } else { border }
}

pub fn render_cpu_card(f: &mut Frame, area: Rect, app: &App, card_w: u16, border_color: Color) {
    let theme = &*app.theme;
    let cpu_pct = app.cpu_spring.value.clamp(0.0, 100.0);
    let cpu_bar = draw_spring_bar(card_w, cpu_pct, 100.0);
    let cpus_count = app.sys.cpus().len();
    let lines = vec![
        Line::from(vec![
            Span::styled("Load:   ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:5.1}%", cpu_pct),
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(cpu_bar, Style::default().fg(Color::Rgb(255, 0, 127))),
            Span::styled("]", Style::default().fg(theme.border)),
        ]),
        Line::from(vec![
            Span::styled("Cores:  ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{} logical", cpus_count), Style::default().fg(theme.text_main)),
        ]),
        Line::from(vec![
            Span::styled("Arch:   ", Style::default().fg(theme.text_dim)),
            Span::styled(std::env::consts::ARCH, Style::default().fg(theme.text_main)),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" CPU ")
        .title_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(border_color));
    f.render_widget(Paragraph::new(lines).block(block), area);
}

pub fn render_memory_card(f: &mut Frame, area: Rect, app: &App, card_w: u16, border_color: Color) {
    let theme = &*app.theme;
    let mem_pct = app.mem_spring.value.clamp(0.0, 100.0);
    let mem_bar = draw_spring_bar(card_w, mem_pct, 100.0);
    let total_mem_gb = app.sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem_gb = app.sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let pink = Color::Rgb(255, 121, 198);
    let lines = vec![
        Line::from(vec![
            Span::styled("Usage:  ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:5.1}%", mem_pct), Style::default().fg(pink).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(mem_bar, Style::default().fg(pink)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]),
        Line::from(vec![
            Span::styled("Used:   ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:.1} GB", used_mem_gb), Style::default().fg(theme.text_main)),
        ]),
        Line::from(vec![
            Span::styled("Total:  ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:.1} GB", total_mem_gb), Style::default().fg(theme.text_main)),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Memory ")
        .title_style(Style::default().fg(pink).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(border_color));
    f.render_widget(Paragraph::new(lines).block(block), area);
}

pub fn render_disk_card(f: &mut Frame, area: Rect, app: &App, card_w: u16, border_color: Color) {
    let theme = &*app.theme;
    let disk_pct = app.disk_spring.value.clamp(0.0, 100.0);
    let disk_bar = draw_spring_bar(card_w, disk_pct, 100.0);
    let mut total_disk: u64 = 0;
    let mut used_disk: u64 = 0;
    for disk in &app.disks {
        total_disk += disk.total_space();
        used_disk += disk.total_space().saturating_sub(disk.available_space());
    }
    let total_disk_gb = total_disk as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_disk_gb = used_disk as f64 / 1024.0 / 1024.0 / 1024.0;
    let purple = Color::Rgb(160, 32, 240);
    let lines = vec![
        Line::from(vec![
            Span::styled("Usage:  ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:5.1}%", disk_pct), Style::default().fg(purple).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(disk_bar, Style::default().fg(purple)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]),
        Line::from(vec![
            Span::styled("Used:   ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:.0} GB", used_disk_gb), Style::default().fg(theme.text_main)),
        ]),
        Line::from(vec![
            Span::styled("Total:  ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:.0} GB", total_disk_gb), Style::default().fg(theme.text_main)),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Storage ")
        .title_style(Style::default().fg(purple).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(border_color));
    f.render_widget(Paragraph::new(lines).block(block), area);
}

pub fn render_gpu_card(
    f: &mut Frame,
    area: Rect,
    app: &App,
    card_w: u16,
    card_w_usize: usize,
    border_color: Color,
) {
    let theme = &*app.theme;
    let gold = Color::Rgb(255, 215, 0);
    let mut gpu_text: Vec<Line> = Vec::new();
    if app.gpu_names.is_empty() {
        let pct = app.gpu1_spring.value.clamp(0.0, 100.0);
        let bar = draw_spring_bar(card_w, pct, 100.0);
        gpu_text.push(format_gpu_line("GPU", "Graphics Engine", pct, card_w_usize));
        gpu_text.push(Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(bar, Style::default().fg(gold)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]));
    } else if app.gpu_names.len() == 1 {
        let pct = app.gpu1_spring.value.clamp(0.0, 100.0);
        let bar = draw_spring_bar(card_w, pct, 100.0);
        gpu_text.push(format_gpu_line("GPU1", &app.gpu_names[0], pct, card_w_usize));
        gpu_text.push(Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(bar, Style::default().fg(gold)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]));
    } else if app.gpu_names.len() == 2 {
        let p1 = app.gpu1_spring.value.clamp(0.0, 100.0);
        let bar1 = draw_spring_bar(card_w, p1, 100.0);
        gpu_text.push(format_gpu_line("GPU1", &app.gpu_names[0], p1, card_w_usize));
        gpu_text.push(Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(bar1, Style::default().fg(gold)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]));
        let p2 = app.gpu2_spring.value.clamp(0.0, 100.0);
        let bar2 = draw_spring_bar(card_w, p2, 100.0);
        gpu_text.push(format_gpu_line("GPU2", &app.gpu_names[1], p2, card_w_usize));
        gpu_text.push(Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(bar2, Style::default().fg(gold)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]));
    } else {
        for (idx, name) in app.gpu_names.iter().enumerate().take(4) {
            let pct = if idx == 0 {
                app.gpu1_spring.value
            } else if idx == 1 {
                app.gpu2_spring.value
            } else {
                let factor = 0.2 + (idx as f64 * 0.12);
                (app.gpu1_spring.value * factor).clamp(1.0, 100.0)
            };
            gpu_text.push(format_gpu_line(
                &format!("GPU{}", idx + 1),
                name,
                pct,
                card_w_usize,
            ));
        }
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" GPU ")
        .title_style(Style::default().fg(gold).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(border_color));
    f.render_widget(Paragraph::new(gpu_text).block(block), area);
}

pub fn render_network_card(f: &mut Frame, area: Rect, app: &App, card_w: u16, border_color: Color) {
    let theme = &*app.theme;
    let net_pct = app.net_spring.value.clamp(0.0, 100.0);
    let net_bar = draw_spring_bar(card_w, net_pct, 100.0);
    let green = Color::Rgb(80, 250, 123);
    let lines = vec![
        Line::from(vec![
            Span::styled("Usage:  ", Style::default().fg(theme.text_dim)),
            Span::styled(format!("{:5.1}%", net_pct), Style::default().fg(green).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("[", Style::default().fg(theme.border)),
            Span::styled(net_bar, Style::default().fg(green)),
            Span::styled("]", Style::default().fg(theme.border)),
        ]),
        Line::from(vec![
            Span::styled("Down:   ", Style::default().fg(theme.text_dim)),
            Span::styled(format_speed(app.rx_speed), Style::default().fg(theme.text_main)),
        ]),
        Line::from(vec![
            Span::styled("Up:     ", Style::default().fg(theme.text_dim)),
            Span::styled(format_speed(app.tx_speed), Style::default().fg(theme.text_main)),
        ]),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Network ")
        .title_style(Style::default().fg(green).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(border_color));
    f.render_widget(Paragraph::new(lines).block(block), area);
}
