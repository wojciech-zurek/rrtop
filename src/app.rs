use crate::colorscheme::ColorScheme;
use crate::widget::menu::Menu;
use crate::widget::status_bar::StatusBar;
use crate::widget::network::Network;

pub struct App<'a> {
    pub menu: Menu<'a>,
    pub status_bar: StatusBar<'a>,
    pub network: Network<'a>,
    pub selected_tab: usize,
}

impl<'a> App<'a> {
    pub fn new(color_scheme: &'a ColorScheme) -> Self {
        App {
            menu: Menu::new(color_scheme),
            status_bar: StatusBar::new(color_scheme),
            network: Network::new(color_scheme),
            selected_tab: 0,
        }
    }

    pub fn on_tab(&mut self) {
        self.selected_tab += 1;
        if self.selected_tab > self.menu.titles.len() - 1 {
            self.selected_tab = 0
        }

        self.menu.select(self.selected_tab)
    }
}

pub struct Widget<'a, T: tui::widgets::Widget> {
    widget: &'a T,
}

pub struct StatefulWidget<T: tui::widgets::StatefulWidget> {
    widget: T,
    state: T::State,
}

