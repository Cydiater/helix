use crate::theme::Theme;
use crate::{Document, View};

use std::path::PathBuf;

use anyhow::Error;

pub struct Editor {
    pub views: Vec<View>,
    pub focus: usize,
    pub should_close: bool,
    pub theme: Theme, // TODO: share one instance
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

impl Editor {
    pub fn new() -> Self {
        let theme = Theme::default();

        Self {
            views: Vec::new(),
            focus: 0,
            should_close: false,
            theme,
        }
    }

    pub fn open(&mut self, path: PathBuf, size: (u16, u16)) -> Result<(), Error> {
        let pos = self.views.len();
        let doc = Document::load(path, self.theme.scopes())?;
        self.views.push(View::new(doc, size)?);
        self.focus = pos;
        Ok(())
    }

    pub fn view(&self) -> Option<&View> {
        self.views.get(self.focus)
    }

    pub fn view_mut(&mut self) -> Option<&mut View> {
        self.views.get_mut(self.focus)
    }
}