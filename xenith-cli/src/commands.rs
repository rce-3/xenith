/*
Xenith - Xen-based security hypervisor
Copyright (C) 2025-2026 Xenith contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

//! Xenith main command handler
//!
//! This module contains the main command handler for the Xenith CLI.
//! All subcommands are listed in the [`Commands`] enum with their own
//! arguments.
//!
//! All commands are handled here and dispatched to the appropriate
//! command handler, each in their own module through the [`handle`] function.
//! This allows for easy extensibility and maintainability of the CLI.

mod vm;

use crate::commands::vm::VmArgs;

use anstyle::{AnsiColor, Color, Style};
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Xenith CLI
///
/// A CLI tool for interacting with Xen and Xenith tools, this tool is designed to be a
/// single entry point for those tools.
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "xenith")]
#[command(version)]
#[command(about = "A CLI tool for interacting with Xen and Xenith tools", long_about = None)]
#[command(help_template = "\
{before-help}Xenith v{version}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
#[command(styles=xenith_styles())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[command(flatten)]
    pub verbosity: Verbosity<InfoLevel>,
}

/// Commands for the CLI
///
/// The commands that can be run, this is the top level of the CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Interact with VMs")]
    Vm(VmArgs),
}

/// Handle the CLI command
///
/// This function takes the CLI arguments and dispatches them to the
/// appropriate command handler.
///
/// # Arguments
///
/// * `args` - The `clap` CLI arguments
pub fn handle(args: Cli) {
    match args.command {
        Commands::Vm(args) => vm::handle(args),
    }
}

/// Get the styles for the CLI
///
/// # Returns
///
/// The styles for the CLI
fn xenith_styles() -> clap::builder::Styles {
    let bold_underline_yellow = Style::new()
        .bold()
        .underline()
        .fg_color(Some(Color::Ansi(AnsiColor::Yellow)));
    let bold_red = Style::new()
        .bold()
        .fg_color(Some(Color::Ansi(AnsiColor::Red)));
    let green = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));
    let bold_underline_green = Style::new()
        .bold()
        .underline()
        .fg_color(Some(Color::Ansi(AnsiColor::Green)));
    let white = Style::new().fg_color(Some(Color::Ansi(AnsiColor::White)));

    clap::builder::Styles::styled()
        .usage(bold_underline_yellow)
        .header(bold_underline_yellow)
        .literal(green)
        .invalid(bold_red)
        .error(bold_red)
        .valid(bold_underline_green)
        .placeholder(white)
}
