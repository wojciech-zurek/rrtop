use crate::colorscheme::ColorScheme;
use crate::widget::menu::Menu;
use crate::widget::status_bar::StatusBar;
use crate::widget::network::Network;
use crate::widget::throughput::Throughput;
use crate::widget::cpu::Cpu;
use crate::widget::memory::Memory;
use crate::widget::stat::Stat;
use tui::style::Style;
use tui::widgets::TableState;

pub struct App<'a> {
    pub menu: Menu<'a>,
    pub status_bar: StatusBar<'a>,
    pub network: Network<'a>,
    pub throughput: Throughput<'a>,
    pub cpu: Cpu<'a>,
    pub memory: Memory<'a>,
    pub stat: Stat<'a>,
    pub stat_table_state: TableState,
    pub selected_tab: usize,
    pub draw_background: Option<Style>,
}

impl<'a> App<'a> {
    pub fn new(color_scheme: &'a ColorScheme, tick_rate: u64, draw_background: Option<Style>) -> Self {
        App {
            menu: Menu::new(color_scheme),
            status_bar: StatusBar::new(color_scheme),
            network: Network::new(color_scheme),
            throughput: Throughput::new(color_scheme),
            cpu: Cpu::new(color_scheme, tick_rate),
            memory: Memory::new(color_scheme, tick_rate),
            stat: Stat::new(color_scheme, tick_rate),
            stat_table_state: TableState::default(),
            selected_tab: 0,
            draw_background,
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
