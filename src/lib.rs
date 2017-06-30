//! # Intro
//! `term_cursor` is a crate for handling terminal cursor movement in a platform independent way.
//! The supported platforms are:
//!
//! - `Windows`
//! - `Linux`
//! - `OS X` (not tested)
//! - `FreeBSD` (not tested)
//! - `OpenBSD` (not tested)
//!
//! # API
//! This crate provides 2 APIs which can be used to achieve the same effects:
//!
//! - A functions based approach, which provides very simple functions to directly interact with the terminal (see the functions section below).
//! - A newtype pattern based approach, that provies a bunch of types which all implement `std::fmt::Display` (see the structs section below).
//! When such types get formatted, they operate on the terminal in a way very similar to the functions API.
//!
//! # Watch out!
//! Both APIs **always** operate on the "default" terminal that is bound to the process.
//! In other words, on Windows `GetStdHandle(STD_OUTPUT_HANDLE)` is used, and on *NIX, the ANSI terminal communcation is done through `stdout` / `stdin`.

use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;
use std::fmt::Error as FmtError;

mod platform;

/// The central error type.
pub enum Error {
    IoError(std::io::Error),
    GetCursorPosParseError,
    #[cfg(target_os = "windows")]
    WinApiError(WinApiError),

}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

/// A windows specific error. Indicates, which `WinAPI` call failed.
#[cfg(target_os = "windows")]
pub enum WinApiError {
    GetStdHandleError,
    GetConsoleScreenBufferInfoError,
    FillConsoleOutputCharacterError,
    FillConsoleOutputAttributeError,
    SetConsoleCursorPositionError,
}

/// A type that, when `Display`ed, makes the cursor go the specified coordinates.
///
/// The Goto tuple type takes a tuple of the form (x, y), where X and Y correspond to columns and rows respectively.
#[derive(Clone, Copy)]
pub struct Goto(pub i32, pub i32);

impl Display for Goto {
    fn fmt(&self, _fmt: &mut Formatter) -> FmtResult {
        let Goto(x, y) = *self;
        platform::set_cursor_pos(x, y).map_err(|_| FmtError)?;
        Ok(())
    }
}

/// A type that, when `Display`ed, makes the cursor move by the specified amount.
///
/// This is identical to a `Goto(get_cursor_pos() + (x, y))` (pseudocode).
///
/// The `Relative` tuple type takes (x, y) coordinates, where X and Y correspond to columns and rows respectively.
#[derive(Clone, Copy)]
pub struct Relative(pub i32, pub i32);

impl Display for Relative {
    fn fmt(&self, _fmt: &mut Formatter) -> FmtResult {
        let (cur_x, cur_y) = platform::get_cursor_pos().map_err(|_| FmtError)?;
        let Relative(x, y) = *self;
        let (x, y) = (x + cur_x, y + cur_y);
        platform::set_cursor_pos(x, y).map_err(|_| FmtError)?;
        Ok(())
    }
}

/// A type that, when `Display`ed, makes the cursor move left by the specified amount.
///
/// This is identical to a `Relative(-val, 0)`.
#[derive(Clone, Copy)]
pub struct Left(pub i32);

impl Display for Left {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Relative(-self.0, 0).fmt(fmt)
    }
}

/// A type that, when `Display`ed, makes the cursor move right by the specified amount.
///
/// This is identical to a `Relative(val, 0)`.
#[derive(Clone, Copy)]
pub struct Right(pub i32);

impl Display for Right {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Relative(self.0, 0).fmt(fmt)
    }
}

/// A type that, when `Display`ed, makes the cursor move up by the specified amount.
///
/// This is identical to a `Relative(0, -val)`.
#[derive(Clone, Copy)]
pub struct Up(pub i32);

impl Display for Up {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Relative(0, -self.0).fmt(fmt)
    }
}

/// A type that, when `Display`ed, makes the cursor move down by the specified amount.
///
/// This is identical to a `Relative(0, val)`.
#[derive(Clone, Copy)]
pub struct Down(pub i32);

impl Display for Down {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Relative(0, self.0).fmt(fmt)
    }
}

/// A type that, when `Display`ed, clears the entire terminal screen.
///
/// In effect, this sets every terminal cell to a space `' '`.
#[derive(Clone, Copy)]
pub struct Clear;

impl Display for Clear {
    fn fmt(&self, _fmt: &mut Formatter) -> FmtResult {
        platform::clear().map_err(|_| FmtError)?;
        Ok(())
    }
}

/// Set the cursor position to the specified coordinates.
/// X and Y correspond to columns and rows respectively.
///
/// ---
/// This function could fail for a number of reasons, depending on the OS.
pub fn set_cursor_pos(x: i32, y: i32) -> Result<(), Error> {
    platform::set_cursor_pos(x, y)
}

/// Get the current cursor position.
/// The tuple returned contains the (x, y) coordinates of the cursor position.
/// X and Y correspond to columns and rows respectively.
///
/// ---
/// This function could fail for a number of reasons, depending on the OS.
pub fn get_cursor_pos() -> Result<(i32, i32), Error> {
    platform::get_cursor_pos()
}

/// Clear the screen, i.e. setting every character in the terminal to a space `' '`.
pub fn clear() -> Result<(), Error> {
    platform::clear()
}