pub mod customizations;
pub mod state;

use std::borrow::Cow;

pub use customizations::Customizations;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use ratatui::{
    prelude::{Backend, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, List as TuiList, ListItem},
    Frame,
};
use textwrap::{wrap, Options};

use self::{customizations::BorderStyle, state::State};
use crate::{
    interface::ui::{colors::CellColor, Vector},
    parse::MenuOption,
};

pub struct List<'l> {
    pub state: State,
    pub dimensions: Vector,
    data: &'l [MenuOption],
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
        let dimensions = Vector { x: width + border_size, y: height + border_size };

        let area = None;

        Self { state, dimensions, data, customizations, area }
    }
    fn create_widget(
        options: &'l [MenuOption],
        width: u16,
        Customizations { colorscheme, border_style }: &Customizations,
        query: Option<&str>,
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
            .map(|text| Self::create_item(text, width - border_size, colorscheme.key, query))
            .collect::<Vec<_>>();
        let block = border_style.apply(Block::default().style(style).border_style(border_color));
        TuiList::new(items).highlight_style(highlight_style).block(block)
    }
    fn create_item(
        option: &'l MenuOption,
        width: u16,
        key_color: CellColor,
        query: Option<&str>,
    ) -> ListItem<'l> {
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
        let mut wrapped_display =
            wrap.iter().map(|line| Self::style_display(line.clone(), query, display_style));
        // wrap.iter().map(|line| vec![Span::styled(line.clone(), display_style)]);
        let mut first_line =
            vec![Span::styled(format!(" {key} "), key_style), Span::styled(" ", display_style)];
        first_line.extend(wrapped_display.next().unwrap());

        let mut text = Vec::with_capacity(2);
        text.push(Line::from(first_line));
        text.extend(wrapped_display.map(Line::from));

        ListItem::new(text)
    }
    fn style_display<'s>(string: Cow<'s, str>, query: Option<&str>, style: Style) -> Vec<Span<'s>> {
        let matches = |query| SkimMatcherV2::default().fuzzy_indices(&string, query);
        if let Some((_, indices)) = query.and_then(matches) {
            let highlight_style = style.red();
            string
                .char_indices()
                .map(|(index, character)| {
                    if indices.contains(&index) {
                        Span::styled(String::from(character), highlight_style)
                    }
                    else {
                        Span::styled(String::from(character), style)
                    }
                })
                .collect()
        }
        else {
            vec![Span::styled(string, style)]
        }
    }
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect, query: Option<&str>) {
        let widget = Self::create_widget(self.data, area.width, &self.customizations, query);
        let state = &mut self.state.inner;

        frame.render_stateful_widget(widget, area, state);
        self.area = Some(area);
    }
    pub fn select(&mut self, coordinate: Vector) -> Option<usize> {
        let position = self.area.and_then(|area| Self::row_in_area(area, coordinate));
        if position == self.state.selected() {
            position
        }
        else {
            self.state.inner.select(position);
            None
        }
    }
    fn row_in_area(area: Rect, Vector { x, y }: Vector) -> Option<usize> {
        (area.x ..= area.x + area.width).contains(&x).then_some((y - area.y - 1) as usize)
    }
    pub fn query(&mut self, term: &str) {
        let matcher = SkimMatcherV2::default();
        let score = |subject: String| matcher.fuzzy_match(&subject, term);
        let max = |holder: Option<(usize, i64)>, contender| {
            holder.zip(contender).map_or_else(
                || holder.or(contender),
                |(holder, contender)| {
                    if contender.1 > holder.1 {
                        Some(contender)
                    }
                    else {
                        Some(holder)
                    }
                },
            )
        };
        let index = self
            .data
            .iter()
            .enumerate()
            .map(|(index, option)| score(option.to_string()).map(|score| (index, score)))
            .reduce(max)
            .unwrap()
            .map(|(index, _)| index);
        self.state.inner.select(index);
    }
}

fn options_width(options: &[MenuOption]) -> u16 {
    let to_lengths = |option: &MenuOption| option.to_string().chars().count();

    let key_chars = 3;
    let display_chars = options.iter().map(to_lengths).max().unwrap() + 1;

    (display_chars + key_chars).try_into().unwrap()
}
