use std::ops::{Range, RangeInclusive};

use super::*;
use druid_shell::Cursor;
use guion::style::standard::cursor::StdCursor;
use guion::text::cursel::{TxtCurSelBytePos, TxtCurSel};
use guion::text::layout::TxtLayout;

pub trait IntoGuionDruidShellCursor<E> {
    fn into_druid_shell_cursor(self) -> Cursor;
}

impl<T,E> IntoGuionDruidShellCursor<E> for T where T: Into<StdCursor> {
    fn into_druid_shell_cursor(self) -> Cursor {
        match Into::<StdCursor>::into(self) {
            StdCursor::Default => Cursor::Arrow,
            StdCursor::Arrow => Cursor::Arrow,
            StdCursor::IBeam => Cursor::IBeam,
            StdCursor::Wait => Cursor::Arrow, //TODO
            StdCursor::Crosshair => Cursor::Crosshair,
            StdCursor::WaitArrow => Cursor::Arrow, //TODO
            StdCursor::SizeNWSE => Cursor::Arrow, //TODO
            StdCursor::SizeNESW => Cursor::Arrow, //TODO
            StdCursor::SizeWE => Cursor::ResizeLeftRight,
            StdCursor::SizeNS => Cursor::ResizeUpDown,
            StdCursor::SizeAll => Cursor::Arrow, //TODO
            StdCursor::No => Cursor::NotAllowed,
            StdCursor::Hand => Cursor::Pointer,
            _ => Cursor::Arrow,
        }
    }
}

#[derive(Copy,Clone,Default)]
pub struct Cusror {
    pub select: u32,
    pub caret: u32,
    pub cursor_stick_x: Option<u32>,
}

impl Cusror {
    pub fn min(&self, min: u32) -> Self {
        Self{
            select: self.select.min(min),
            caret: self.caret.min(min),
            cursor_stick_x: None,
        }
    }
    pub fn range(&self) -> Range<u32> {
        self.select.min(self.caret) .. self.select.max(self.caret)
    }
    pub fn range_usize(&self) -> Range<usize> {
        self.select.min(self.caret) as usize .. self.select.max(self.caret) as usize
    }
    pub fn range_incl(&self) -> RangeInclusive<u32> {
        self.select.min(self.caret) ..= self.select.max(self.caret)
    }
    pub fn start_len(&self) -> (u32,u32) {
        let r = self.range();
        (r.start,r.end-r.start)
    }
    pub fn is_selection(&self) -> bool {
        self.select != self.caret
    }
    pub fn unselect(&mut self) {
        self.select = self.caret;
    }
    pub fn unselect_add(&mut self, o: u32, skip_unselect: bool) {
        self.caret += o;
        if !skip_unselect {
            self.select = self.caret;
        }
    }
    pub fn unselect_sub(&mut self, o: u32, skip_unselect: bool) {
        self.caret = self.caret.saturating_sub(o);
        if !skip_unselect {
            self.select = self.caret;
        }
    }
    pub fn unselect_addi(&mut self, o: i32, skip_unselect: bool) {
        self.caret = (self.caret as i32 +o).max(0) as u32;
        if !skip_unselect {
            self.select = self.caret;
        }
    }
    pub fn limit(&mut self, min: u32) {
        *self = self.min(min);
    }
    // pub fn del_selection<'a,S,E>(&mut self, c: &mut S) where S: TextStorMut<E>+'a, E: Env {
    //     let (start,len) = self.start_len();
    //     c.remove_chars(self.range_usize());
    //     self.caret = start;
    //     self.unselect();
    // }
}

impl<E> TxtCurSel<E> for Cusror {
    type Cachor = (u32,u32);

    fn cachor(&self) -> Self::Cachor {
        (self.caret,self.select)
    }

    fn typ(&self) -> TxtCurSelBytePos {
        if self.caret == self.select {
            TxtCurSelBytePos::Cursor(self.caret as usize)
        } else {
            TxtCurSelBytePos::Selection(self.range_usize())
        }
    }

    fn is_cursor(&self) -> bool {
        self.caret == self.select
    }

    fn is_selection(&self) -> bool {
        self.caret != self.select
    }

    fn unselect(&mut self) {
        self.select = self.caret;
    }

    fn caret(&self) -> usize {
        self.caret as usize
    }

    fn sync_replace(&mut self, replace_range: Range<usize>, insert: &str) {
        todo!()
    }

    fn attempt_insert_text(&self, insert_len_bytes: usize, base_text_len: usize) -> (usize,Self) {
        (self.caret as usize,Cusror{select:self.caret+insert_len_bytes as u32,caret:self.caret+insert_len_bytes as u32,cursor_stick_x:None})
    }

    fn attempt_replace_text(&self, replacant_len_bytes: usize, base_text_len: usize) -> (Range<usize>,Self) {
        let new_caret = self.range_usize().start + replacant_len_bytes;
        (self.range_usize(),Cusror{select:new_caret as u32,caret:new_caret as u32,cursor_stick_x:None})
    }

    fn attempt_backspace(&self, backspace_bytes: usize, base_text_len: usize) -> (Range<usize>,Self) {
        let off = self.caret as usize;
        let popable = backspace_bytes.min(off).min(base_text_len);
        let pop_start = off - popable;
        let pop_end = off;

        assert!(pop_end >= pop_start);

        (pop_start..pop_end,Cusror{select:pop_start as u32,caret:pop_start as u32,cursor_stick_x:None})
    }
}
