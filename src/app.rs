use tui::style::Style;
use tui::widgets::TableState;

use crate::colorscheme::theme::Theme;
use crate::widget::area_warning::AreaWarning;
use crate::widget::calls::Calls;
use crate::widget::cpu::Cpu;
use crate::widget::hit_rate::HitRate;
use crate::widget::memory::Memory;
use crate::widget::menu::Menu;
use crate::widget::Navigation;
use crate::widget::network::Network;
use crate::widget::raw::Raw;
use crate::widget::stat::Stat;
use crate::widget::status_bar::StatusBar;

const MAIN: &str = "Main";
const CMD: &str = "Cmd";
const STAT: &str = "Stat";
const RAW: &str = "Raw";

pub struct App<'a, 'b> {
    pub menu: Menu<'a, 'b>,
    pub status_bar: StatusBar<'a>,
    pub network: Network<'a>,
    pub cpu: Cpu<'a>,
    pub memory: Memory<'a>,
    pub hit_rate: HitRate<'a>,
    pub stat: Stat<'a>,
    pub calls: Calls<'a>,
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
            menu: Menu::new(theme, vec![MAIN, CMD, STAT, RAW]),
            status_bar: StatusBar::new(theme),
            network: Network::new(theme),
            cpu: Cpu::new(theme),
            memory: Memory::new(theme),
            hit_rate: HitRate::new(theme),
            stat: Stat::new(theme),
            calls: Calls::new(theme),
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
        // match self.selected_tab {
        //     0 => Navigation::prev(&mut self.stat),
        //     1 => {}
        //     2 => Navigation::prev(&mut self.stat),
        //     3 => Navigation::prev(&mut self.raw),
        //     _ => {}
        // };
    }

    pub fn on_key_down(&mut self) {
        // match self.selected_tab {
        //     0 => Navigation::next(&mut self.stat),
        //     1 => {}
        //     2 => self.stat.next(),
        //     3 => self.raw.next(),
        //     _ => {}
        // };
    }
}
