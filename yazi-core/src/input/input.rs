use std::ops::{Deref, DerefMut};

use anyhow::Result;
use yazi_config::popup::Position;
use yazi_macro::{render, succ};
use yazi_shared::{data::Data, SStr};
use yazi_widgets::input::{InputOp, parser::HistoryOpt};
use crate::input::InputHistory;

#[derive(Default)]
pub struct Input {
	pub(super) inner: yazi_widgets::input::Input,
	pub history: std::collections::HashMap<SStr, InputHistory>,

	pub visible:  bool,
	pub title:    String,
	pub position: Position,
}

impl Input {
	pub fn history(&mut self) -> &mut InputHistory {
		if !self.history.contains_key(&self.inner.id) {
			self.history.insert(self.inner.id.clone(), InputHistory::new());
		}
		self.history.get_mut(&self.inner.id).unwrap()
	}

	pub fn reset_history(&mut self) {
		if let Some(history) = self.history.get_mut(&self.inner.id) {
			history.reset();
		}
	}

	pub fn navigate_history(&mut self, opt: HistoryOpt) -> Result<Data> {
		if self.inner.snap().op != InputOp::None || self.inner.obscure {
			succ!();
		}
		match self.history.get_mut(&self.inner.id) {
			Some(history) => {
				if !history.navigate(opt.offset, &mut self.inner.snaps, self.inner.limit) {
					succ!();
				}
			}
			None => succ!(),
		}
		self.inner.flush_type();
		succ!(render!());
	}
}

impl Deref for Input {
	type Target = yazi_widgets::input::Input;

	fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for Input {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}
