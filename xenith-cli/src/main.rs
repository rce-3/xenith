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

use clap::Parser;

mod commands;

use commands::{Cli, handle};

fn main() {
    let args = Cli::parse();

    // Initialize the logger
    let log_level = args.verbosity.log_level_filter();
    let mut clog = colog::default_builder();
    clog.filter(None, log_level);
    clog.init();

    // Handle CLI commands
    handle(args);
}
