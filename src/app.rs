use tui::style::Style;

use crate::colorscheme::theme::Theme;
use crate::widget::area_warning::AreaWarning;
use crate::widget::calls::Calls;
use crate::widget::cpu::Cpu;
use crate::widget::memory::Memory;
use crate::widget::menu::Menu;
use crate::widget::navigation::Navigation;
use crate::widget::network::Network;
use crate::widget::raw::Raw;
use crate::widget::slow_log::SlowLog;
use crate::widget::stat::Stat;
use crate::widget::status_bar::StatusBar;

const MAIN: &str = "Main";
const CMD: &str = "Cmd";
const STAT: &str = "Stat";
const SLOW: &str = "Slow";
const RAW: &str = "Raw";

pub struct App<'a, 'b> {
    pub menu: Menu<'a, 'b>,
    pub status_bar: StatusBar<'a>,
    pub network: Network<'a>,
    pub cpu: Cpu<'a>,
    pub memory: Memory<'a>,
    pub stat: Stat<'a>,
    pub calls: Calls<'a>,
    pub slow_log: SlowLog<'a>,
    pub raw: Raw<'a>,
    pub area_warning: AreaWarning<'a>,
    pub selected_tab: usize,
    pub draw_background: Option<Style>,
    pub min_width: u16,
    pub min_height: u16,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new(theme: &'a Theme, draw_background: Option<Style>, min_width: u16, min_height: u16) -> Self {
        App {
            menu: Menu::new(theme, vec![MAIN, CMD, STAT, SLOW, RAW]),
            status_bar: StatusBar::new(theme),
            network: Network::new(theme),
            cpu: Cpu::new(theme),
            memory: Memory::new(theme),
            stat: Stat::new(theme),
            calls: Calls::new(theme),
            slow_log: SlowLog::new(theme),
            raw: Raw::new(theme),
            area_warning: AreaWarning::new(theme, min_width, min_height),
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

    pub fn on_sort(&mut self) {
        self.calls.sort_next();
    }

    pub fn on_key_up(&mut self) {
        match self.selected_tab {
            0 => self.stat.prev(),
            1 => self.calls.prev(),
            2 => self.stat.prev(),
            3 => self.slow_log.prev(),
            4 => self.raw.prev(),
            _ => {}
        };
    }

    pub fn on_key_down(&mut self) {
        match self.selected_tab {
            0 => self.stat.next(),
            1 => self.calls.next(),
            2 => self.stat.next(),
            3 => self.slow_log.next(),
            4 => self.raw.next(),
            _ => {}
        };
    }
}
