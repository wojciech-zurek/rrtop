use tui::style::Style;
use tui::widgets::TableState;

use crate::colorscheme::theme::Theme;
use crate::widget::area_warning::AreaWarning;
use crate::widget::calls::Calls;
use crate::widget::cpu::Cpu;
use crate::widget::hit_rate::HitRate;
use crate::widget::memory::Memory;
use crate::widget::menu::Menu;
use crate::widget::network::Network;
use crate::widget::stat::Stat;
use crate::widget::status_bar::StatusBar;

pub struct App<'a> {
    pub menu: Menu<'a>,
    pub status_bar: StatusBar<'a>,
    pub network: Network<'a>,
    pub cpu: Cpu<'a>,
    pub memory: Memory<'a>,
    pub hit_rate: HitRate<'a>,
    pub stat: Stat<'a>,
    pub commands: Calls<'a>,
    pub area_warning: AreaWarning<'a>,
    pub stat_table_state: TableState,
    pub selected_tab: usize,
    pub draw_background: Option<Style>,
    pub min_width: u16,
    pub min_height: u16,
}

impl<'a, 'b> App<'a> {
    pub fn new(theme: &'a Theme, draw_background: Option<Style>, min_width: u16, min_height: u16) -> Self {
        App {
            menu: Menu::new(theme),
            status_bar: StatusBar::new(theme),
            network: Network::new(theme),
            cpu: Cpu::new(theme),
            memory: Memory::new(theme),
            hit_rate: HitRate::new(theme),
            stat: Stat::new(theme),
            commands: Calls::new(theme),
            area_warning: AreaWarning::new(theme, min_width, min_height),
            stat_table_state: TableState::default(),
            selected_tab: 0,
            draw_background,
            min_width,
            min_height,
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
