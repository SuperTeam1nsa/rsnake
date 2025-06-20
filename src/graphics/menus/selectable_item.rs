use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Widget},
};
///TODO finish it to have a graphical option setup alongside CLI !
/// Generic selectable list for Ratatui
pub struct SelectableList<T> {
    pub title: String,
    pub items: Vec<T>,
    pub selected: usize,
    pub fg: Color,
    pub bg: Color,
    formatter: fn(&T) -> String,
}

impl<T> SelectableList<T> {
    pub fn new(
        title: String,
        items: Vec<T>,
        formatter: fn(&T) -> String,
        fg: Color,
        bg: Color,
    ) -> Self {
        Self {
            title,
            items,
            selected: 0,
            fg,
            bg,
            formatter,
        }
    }

    pub fn next(&mut self) {
        self.selected = (self.selected + 1) % self.items.len();
    }

    pub fn prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn selected_item(&self) -> &T {
        &self.items[self.selected]
    }

    pub fn update_selected(&mut self, value: T) {
        self.items[self.selected] = value;
    }
}

impl<T> Widget for &SelectableList<T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let content = (self.formatter)(item);
                let style = if i == self.selected {
                    Style::default().fg(self.bg).bg(self.fg)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(Span::styled(content, style)))
            })
            .collect();

        let list = List::new(list_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.clone()),
        );

        list.render(area, buf);
    }
}
