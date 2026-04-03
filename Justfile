# Main Justfile for Xenith
#
# Provides a simple interface to run various tasks as well as
# useful variables.
#
# Provides:
#   - `ROOT_DIR`: The root directory of the project.
#   - `PACKER_DIR`: The directory where Packer templates are stored.
#   - `ANSIBLE_DIR`: The directory where Ansible roles and playbooks are stored.
#   - `WEBSITE_DIR`: The directory where the website files are stored.
#
# Check Just documentation for more information:
# https://just.systems/man/en/introduction.html


# ------------------------------
# Settings
# ------------------------------
set shell := ["/usr/bin/env", "bash", "-c"]
set allow-duplicate-recipes

# ------------------------------
# Requirements
# ------------------------------
cargo := require("cargo")

# ------------------------------
# Variables
# ------------------------------
ROOT_DIR := source_directory()
PACKER_DIR := ROOT_DIR / "packer"
ANSIBLE_DIR := ROOT_DIR / "ansible"
WEBSITE_DIR := ROOT_DIR / "xenith-website"

build_profile := "dev"
build_package := ""

# ------------------------------
# Tasks
# ------------------------------
[doc("Default task - will be run when no task is specified.")]
default:
    @just --list
    @echo
    @echo "Note: you can run recipes from subdirectories with: just <subdir>/<recipe>."
    @echo "For example: just packer/build-image."

[group("xenith")]
[doc("Clean the Cargo build directory."), confirm("Are you sure you want to clean the build directory? (y/n)")]
clean package=build_package:
    {{ cargo }} clean {{ if package == "" { "" } else { "-p " + package } }}

[group("xenith")]
[doc("Format the code using rustfmt.")]
format package=build_package:
    {{ cargo }} fmt {{ if package == "" { "--all" } else { "-p " + package } }} -- --emit=files

[group("xenith")]
[doc("Run strict static analysis using Clippy.")]
lint package=build_package:
    {{ cargo }} clippy --all-targets {{ if package == "" { "--all" } else { "-p " + package } }} -- -D warnings

[group("xenith")]
[doc("Run all tests.")]
test package=build_package:
    {{ cargo }} test {{ if package == "" { "--all" } else { "-p " + package } }} --all-targets

[group("xenith")]
[doc("Run all documentation tests.")]
doctest package=build_package:
    {{ cargo }} test {{ if package == "" { "--all" } else { "-p " + package } }} --doc

[group("xenith")]
[doc("Generate documentation for the project.")]
generate-docs package=build_package:
     RUSTDOCFLAGS="$RUSTDOCFLAGS --enable-index-page -Zunstable-options --default-theme=ayu" {{ cargo }} +nightly doc \
        {{ if package == "" { "--all" } else { "-p " + package } }} \
        --workspace \
        --no-deps \
        --document-private-items

[group("xenith")]
[doc("Build project. Use release parameter for release builds. Use package parameter to build a specific package.")]
build profile=build_profile package=build_package:
    {{ cargo }} build {{ if profile == "release" { "--release" } else { "" } }} --all-targets {{ if package == "" { "--workspace" } else { "-p " + package } }}

[group("xenith")]
[doc("Run the project with configuration file. Use release parameter for release builds. Use package parameter to run a specific package.")]
run profile=build_profile package=build_package: lint format
    {{ cargo }} run {{ if profile == "release" { "--release" } else { "" } }} {{ if package == "" { "" } else { "-p " + package } }}

[group("xen")]
[doc("Update the Xen source code.")]
update-xen:
    @echo "Updating Xen source code..."
    @git submodule update --recursive --remote
    @echo "Xen source code updated."
