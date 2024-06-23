use std::time::Duration;

use color_eyre::{eyre::Context, Result};
use crossterm::event::{
  Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
};
use itertools::Itertools;
use ratatui::{
  backend::Backend,
  buffer::Buffer,
  layout::{Constraint, Layout, Rect},
  style::Color,
  terminal::Terminal,
  text::{Line, Span},
  widgets::{Block, Tabs, Widget},
};
use sea_orm::DatabaseConnection;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::term;

#[derive(Debug, Default, Clone)]
pub struct App {
  mode: Mode,
  tab: Tab,
  db: DatabaseConnection,
}

impl App {
  /// Run the app until the user quits.
  pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
    while self.is_running() {
      self.draw(terminal)?;
      self.handle_events()?;
    }
    Ok(())
  }

  fn is_running(&self) -> bool {
    self.mode != Mode::Quit
  }

  /// Handle events from the terminal.
  ///
  /// This function is called once per frame, The events are polled from the stdin with timeout of
  /// 1/50th of a second. This was chosen to try to match the default frame rate of a GIF in VHS.
  fn handle_events(&mut self) -> Result<()> {
    let timeout = Duration::from_secs_f64(1.0 / 50.0);
    match term::next_event(timeout)? {
      Some(Event::Key(key)) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
      Some(Event::Mouse(mouse)) => {}
      _ => {}
    }
    Ok(())
  }

  fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
    todo!()
  }

  fn handle_key_press(&self, key: KeyEvent) {
    todo!()
  }

  fn handle_mouse_event(&self) {}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
  #[default]
  Running,
  Destroy,
  Quit,
}

#[derive(Debug, Clone, Copy, Default, Display, EnumIter, FromRepr, PartialEq, Eq)]
enum Tab {
  #[default]
  About,
  User,
  Media,
  Tag,
  Comment,
  MediaTag,
  Like,
  Favorite,
  Subscribe,
}
