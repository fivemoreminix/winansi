#![cfg(windows)]

use winapi::um::{
    consoleapi::SetConsoleMode,
    processenv::GetStdHandle,
};

/// Enable virtual terminal processing for the currently running Windows console.
pub fn windows_ansi() {
    // see https://docs.microsoft.com/en-us/windows/console/setconsolemode#ENABLE_VIRTUAL_TERMINAL_PROCESSING
    // and https://stackoverflow.com/a/36760881
    SetConsoleMode(GetStdHandle(-11), 7);
}
