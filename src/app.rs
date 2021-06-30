use crate::colorscheme::ColorScheme;
use crate::widget::menu::Menu;
use crate::widget::status_bar::StatusBar;
use crate::widget::network::Network;
use crate::widget::throughput::Throughput;
use crate::widget::cpu::Cpu;

pub struct App<'a> {
    pub menu: Menu<'a>,
    pub status_bar: StatusBar<'a>,
    pub network: Network<'a>,
    pub throughput: Throughput<'a>,
    pub cpu: Cpu<'a>,
    pub selected_tab: usize,
}

impl<'a> App<'a> {
    pub fn new(color_scheme: &'a ColorScheme, tick_rate: u64) -> Self {
        App {
            menu: Menu::new(color_scheme),
            status_bar: StatusBar::new(color_scheme),
            network: Network::new(color_scheme),
            throughput: Throughput::new(color_scheme),
            cpu: Cpu::new(color_scheme, tick_rate),
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
