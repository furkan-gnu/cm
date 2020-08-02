use super::*;

#[derive(PartialEq)]
pub enum StringListState {
    Navigate,
    Editing { new: bool, prev_cursor_y: usize },
}

pub struct StringList {
    pub state: StringListState,
    pub list: ItemList,
    pub edit_field: EditField,
}

impl StringList {
    pub fn new() -> Self {
        Self {
            state: StringListState::Navigate,
            list: ItemList::new(),
            edit_field: EditField::new(),
        }
    }

    pub fn current_item(&self) -> Option<&str> {
        self.list.current_item()
    }

    pub fn render(&self, rect: Rect, focused: bool, cursor: &mut Cursor) {
        self.list.render(rect, focused);
        if let StringListState::Editing { .. } = self.state {
            let row = self.list.current_row(rect);
            self.edit_field.render(row, cursor);
        }
    }

    pub fn duplicate_after(&mut self) {
        if let StringListState::Navigate = self.state {
            self.list.duplicate_after();
        }
    }

    pub fn duplicate_before(&mut self) {
        if let StringListState::Navigate = self.state {
            self.list.duplicate_before();
        }
    }

    pub fn insert_after(&mut self, cursor: &mut Cursor) {
        if let StringListState::Navigate = self.state {
            self.state = StringListState::Editing {
                new: true,
                prev_cursor_y: self.list.cursor_y,
            };
            self.list.insert_after_current(String::new());
            self.edit_field.buffer.clear();
            self.edit_field.cursor_x = 0;
            cursor.visible = true;
        }
    }

    pub fn insert_before(&mut self, cursor: &mut Cursor) {
        if let StringListState::Navigate = self.state {
            self.state = StringListState::Editing {
                new: true,
                prev_cursor_y: self.list.cursor_y,
            };
            self.list.insert_before_current(String::new());
            self.edit_field.buffer.clear();
            self.edit_field.cursor_x = 0;
            cursor.visible = true;
        }
    }

    pub fn start_editing(&mut self, cursor: &mut Cursor) {
        if let StringListState::Navigate = self.state {
            if let Some(item) = self.list.current_item() {
                self.edit_field.cursor_x = item.len();
                self.edit_field.buffer = String::from(item);
                self.state = StringListState::Editing {
                    new: false,
                    prev_cursor_y: self.list.cursor_y,
                };
                cursor.visible = true;
            }
        }
    }

    pub fn accept_editing(&mut self, cursor: &mut Cursor) {
        if let StringListState::Editing { .. } = self.state {
            self.state = StringListState::Navigate;
            self.list.items[self.list.cursor_y] = self.edit_field.buffer.clone();
            cursor.visible = false;
        }
    }

    pub fn cancel_editing(&mut self, cursor: &mut Cursor) {
        if let StringListState::Editing { new, prev_cursor_y } = self.state {
            self.state = StringListState::Navigate;
            if new {
                self.list.delete_current();
                self.list.cursor_y = prev_cursor_y
            }
            cursor.visible = false;
        }
    }

    pub fn handle_key(
        &mut self,
        key_stroke: &KeyStroke,
        key_map: &KeyMap,
        global: &mut Global,
        cursor: &mut Cursor,
    ) {
        match self.state {
            StringListState::Navigate => {
                if !global.handle_key(key_stroke, key_map) {
                    if key_map.is_bound(key_stroke, &Action::DupAfterItem) {
                        self.duplicate_after();
                    } else if key_map.is_bound(key_stroke, &Action::DupBeforeItem) {
                        self.duplicate_before();
                    } else if key_map.is_bound(key_stroke, &Action::InsertAfterItem) {
                        self.insert_after(cursor);
                    } else if key_map.is_bound(key_stroke, &Action::InsertBeforeItem) {
                        self.insert_before(cursor);
                    } else if key_map.is_bound(key_stroke, &Action::EditItem) {
                        self.start_editing(cursor);
                    } else {
                        self.list.handle_key(key_stroke, key_map);
                    }
                }
            }
            StringListState::Editing { .. } => {
                if key_map.is_bound(key_stroke, &Action::Accept) {
                    self.accept_editing(cursor);
                } else if key_map.is_bound(key_stroke, &Action::Cancel) {
                    self.cancel_editing(cursor);
                } else {
                    self.edit_field.handle_key(key_stroke, key_map);
                }
            }
        }
    }
}