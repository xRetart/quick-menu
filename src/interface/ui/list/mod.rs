pub mod customizations;
pub mod state;

pub use customizations::Customizations;
use ratatui::{
    prelude::{Backend, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List as TuiList, ListItem},
    Frame,
};
use textwrap::{wrap, Options};

use self::{customizations::BorderStyle, state::State};
use super::colorscheme::CellColor;
use crate::{interface::ui::Coordinate, parse::MenuOption};

pub struct List<'l> {
    pub state: State,
    data: &'l [MenuOption],
    dimensions: Coordinate,
    customizations: Customizations,
    area: Option<Rect>,
}

impl<'l> List<'l> {
    pub fn new(data: &'l [MenuOption], customizations: Customizations) -> Self {
        let length = data.len();
        let state = State::with_length(length);

        let border_size =
            if matches!(customizations.border_style, BorderStyle::None) { 0 } else { 2 };
        let width = options_width(data);
        let height = u16::try_from(data.len()).unwrap();
        let dimensions = Coordinate { x: width + border_size, y: height + border_size };

        let area = None;

        Self { state, data, dimensions, customizations, area }
    }
    fn create_widget(
        options: &'l [MenuOption],
        width: u16,
        Customizations { colorscheme, border_style }: &Customizations,
    ) -> TuiList<'l> {
        let border_size = if matches!(border_style, BorderStyle::None) { 0 } else { 2 };
        let style = Style::default();
        let border_color = style.fg(colorscheme.border);
        let highlight_style = style
            .add_modifier(Modifier::BOLD)
            .bg(colorscheme.selected.background)
            .fg(colorscheme.selected.foreground);

        let items = options
            .iter()
            .map(|text| Self::create_item(text, width - border_size, colorscheme.key))
            .collect::<Vec<_>>();
        let block = border_style.apply(Block::default().style(style).border_style(border_color));
        TuiList::new(items).highlight_style(highlight_style).block(block)
    }
    fn create_item(option: &'l MenuOption, width: u16, key_color: CellColor) -> ListItem<'l> {
        let MenuOption { key, output: _, display } = option;

        let default_style = Style::default();
        let display_style = default_style;
        let key_style = default_style
            .add_modifier(Modifier::BOLD)
            .fg(key_color.foreground)
            .bg(key_color.background);

        let wrap = wrap(
            display,
            Options::new(usize::try_from(width - 4).unwrap()).subsequent_indent("    "),
        );
        let mut wrapped_display = wrap.iter().map(|line| Span::styled(line.clone(), display_style));
        let mut text = Vec::with_capacity(2);
        text.push(Line::from(vec![
            Span::styled(format!(" {key} "), key_style),
            Span::styled(" ", display_style),
            wrapped_display.next().unwrap(),
        ]));
        text.extend(wrapped_display.map(|line| Line::from(vec![line])));

        ListItem::new(text)
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, chunk: Rect) {
        let area = self.centered(chunk);
        let widget = Self::create_widget(self.data, area.width, &self.customizations);
        let state = &mut self.state.inner;

        frame.render_stateful_widget(widget, area, state);
        self.area = Some(area);
    }
    fn centered(&self, outer: Rect) -> Rect {
        let width = self.dimensions.x.min(outer.width);
        let height = self.dimensions.y.min(outer.height);

        let x = (outer.width - width) / 2;
        let y = (outer.height - height) / 2;

        Rect { x, y, width, height }
    }
    pub fn select(&mut self, coordinate: Coordinate) -> Option<usize> {
        let position = self.area.and_then(|area| Self::row_in_area(area, coordinate));
        if position == self.state.selected() {
            position
        }
        else {
            self.state.inner.select(position);
            None
        }
    }
    fn row_in_area(area: Rect, Coordinate { x, y }: Coordinate) -> Option<usize> {
        (area.x ..= area.x + area.width).contains(&x).then_some((y - area.y - 1) as usize)
    }
    pub fn query(&mut self, term: &str) {
        let term = term.to_lowercase();
        let matches = |subject: String| subject.to_lowercase().starts_with(&term);
        let index = self.data.iter().position(|option| matches(option.to_string()));
        self.state.inner.select(index);
    }
}

fn options_width(options: &[MenuOption]) -> u16 {
    let to_lengths = |option: &MenuOption| option.to_string().chars().count();

    let key_chars = 3;
    let display_chars = options.iter().map(to_lengths).max().unwrap() + 1;

    (display_chars + key_chars).try_into().unwrap()
}
