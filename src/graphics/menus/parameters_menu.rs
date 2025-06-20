use crate::controls::speed::Speed;
use crate::graphics::menus::selectable_item::SelectableList;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use std::fmt::Display;

///TODO finish it to have a graphical option setup alongside CLI !
///
/// Represents a single option in the multiple choice menu
#[derive(Clone)]
pub struct MenuOption<T: Clone> {
    pub value: T,
    pub label: String,
    pub selected: bool,
}

impl<T: Clone> MenuOption<T> {
    pub fn new(value: T, label: String) -> Self {
        Self {
            value,
            label,
            selected: false,
        }
    }

    pub fn toggle(&mut self) {
        self.selected = !self.selected;
    }
}

impl<T: Clone> Display for MenuOption<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if self.selected { "[X] " } else { "[ ] " };
        write!(f, "{}{}", prefix, self.label)
    }
}

/// A menu with multiple choice options and a save button
pub struct MultipleChoiceMenu<T: Clone> {
    pub title: String,
    pub options: SelectableList<MenuOption<T>>,
    pub has_save_button: bool,
    pub is_on_save_button: bool,
}

impl<T: Clone> MultipleChoiceMenu<T> {
    #[must_use]
    pub fn new(title: String, options: Vec<MenuOption<T>>) -> Self {
        let formatter = |opt: &MenuOption<T>| opt.to_string();

        Self {
            title: title.clone(),
            options: SelectableList::new(title, options, formatter, Color::Green, Color::Black),
            has_save_button: true,
            is_on_save_button: false,
        }
    }

    /// Move to the next option or save button
    pub fn next(&mut self) {
        if self.is_on_save_button {
            self.is_on_save_button = false;
            self.options.selected = 0;
        } else if self.options.selected == self.options.items.len() - 1 && self.has_save_button {
            self.is_on_save_button = true;
        } else {
            self.options.next();
        }
    }

    /// Move to the previous option or save button
    pub fn prev(&mut self) {
        if self.is_on_save_button {
            self.is_on_save_button = false;
            self.options.selected = self.options.items.len() - 1;
        } else if self.options.selected == 0 && self.has_save_button {
            self.is_on_save_button = true;
        } else {
            self.options.prev();
        }
    }

    /// Toggle the currently selected option
    pub fn toggle_selected(&mut self) {
        if !self.is_on_save_button {
            let selected = self.options.selected;
            self.options.items[selected].toggle();
        }
    }

    /// Checks if the save button is selected
    #[must_use]
    pub fn is_save_selected(&self) -> bool {
        self.is_on_save_button
    }

    /// Returns a list of all selected values
    #[must_use]
    pub fn get_selected_values(&self) -> Vec<T> {
        self.options
            .items
            .iter()
            .filter(|opt| opt.selected)
            .map(|opt| opt.value.clone())
            .collect()
    }

    /// Handle user input from the menu
    pub fn handle_input(&mut self, event: &Event) -> bool {
        if let Event::Key(KeyEvent { code, .. }) = event {
            match code {
                KeyCode::Down => self.next(),
                KeyCode::Up => self.prev(),
                KeyCode::Enter | KeyCode::Char(' ') => {
                    if self.is_on_save_button {
                        return true; // The Save button was pressed
                    }
                    self.toggle_selected();
                }
                _ => {}
            }
        }
        false // Save not triggered
    }
}

impl<T: Clone> Widget for &MultipleChoiceMenu<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Create a layout with space for options and save button
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(area);

        // Render the option list
        (&self.options).render(chunks[0], buf);

        // Render the save button
        if self.has_save_button {
            let save_style = if self.is_on_save_button {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green)
            };

            let save_button = Paragraph::new(Line::from(Span::styled(" Save ", save_style)))
                .block(Block::default().borders(Borders::ALL));

            save_button.render(chunks[1], buf);
        }
    }
}

/// Runs a multiple-choice menu in the terminal
///
/// This function sets up a TUI menu where the user can select multiple options
/// and confirm with a save button.
///
/// # Returns
///
/// A vector of selected option strings if saved, or an empty vector if quit without saving
///
/// # Errors
///
/// Returns an error if terminal operations fail, such as entering/leaving alternate screen,
/// enabling/disabling raw mode, or other I/O operations.
pub fn run_multiple_choice_menu() -> Result<Vec<Speed>, std::io::Error> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    // Create menu options
    let options: Vec<MenuOption<Speed>> = vec![
        MenuOption::new(Speed::Slow, format!("{} Slow", Speed::Slow.symbol())),
        MenuOption::new(Speed::Normal, format!("{} Normal", Speed::Normal.symbol())),
        MenuOption::new(Speed::Fast, format!("{} Fast", Speed::Fast.symbol())),
        MenuOption::new(
            Speed::Tremendous,
            format!("{} Tremendous", Speed::Tremendous.symbol()),
        ),
    ];

    // Create a menu
    let mut menu = MultipleChoiceMenu::new("Multiple Choice Menu".to_string(), options);

    // Result to return
    let mut selected_options = Vec::new();

    // Main event loop
    loop {
        terminal.draw(|f| {
            let size = f.area();
            f.render_widget(&menu, size);
        })?;

        if let Ok(event) = crossterm::event::read() {
            // Check for quit
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break;
            }

            // Handle menu input
            let save_triggered = menu.handle_input(&event);

            // If save was triggered, collect selected values and exit
            if save_triggered {
                selected_options = menu.get_selected_values();
                break;
            }
        }
    }

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(selected_options)
}
