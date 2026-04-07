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

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct VmArgs {
    #[command(subcommand)]
    pub command: VmCommands,
}

#[derive(Debug, Subcommand)]
pub enum VmCommands {
    Create(VmCreateArgs),
    Destroy,
    Up,
    Halt,
}

#[derive(Debug, Args)]
pub struct VmCreateArgs {
    #[arg(short, long)]
    test: Option<String>,
}

pub fn handle(args: VmArgs) {
    match args.command {
        VmCommands::Create(create) => {
            log::info!("Creating VM with message: {:?}", create.test);
        }
        VmCommands::Destroy => {
            println!("Destroying VM");
        }
        VmCommands::Up => {
            println!("Starting VM");
        }
        VmCommands::Halt => {
            println!("Halting VM");
        }
    }
}
