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
set allow-duplicate-recipes := true

# ------------------------------
# Requirements
# ------------------------------

cargo := require("cargo")
hugo := require("hugo")
xdg-open := require("xdg-open")

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
# Aliases
# ------------------------------

alias cov := coverage
alias fmt := format
alias check := lint

# ------------------------------
# Tasks
# ------------------------------

[doc("Default task - will be run when no task is specified.")]
default:
    @just --list
    @echo
    @echo "Note: you can run recipes from subdirectories with: just <subdir>/<recipe>."
    @echo "For example: just packer/build-image."

[confirm("Are you sure you want to clean the build directory? (y/n)")]
[doc("Clean the Cargo build directory.")]
clean package=build_package:
    {{ cargo }} clean {{ if package == "" { "" } else { "-p " + package } }}

[doc("Format the code using rustfmt.")]
[group("fmt")]
format package=build_package:
    {{ cargo }} fmt {{ if package == "" { "--all" } else { "-p " + package } }} -- --emit=files

[doc("Run strict static analysis using Clippy.")]
[group("lint")]
lint package=build_package:
    {{ cargo }} clippy \
        --all-targets \
        --all-features \
        {{ if package == "" { "--all" } else { "-p " + package } }}

[doc("Check for unused dependencies.")]
[group("lint")]
udeps:
    RUSTC_BOOTSTRAP=1 {{ cargo }} udeps --all-targets --backend depinfo

[doc("Check for security vulnerabilities, license compliance and others in dependencies.")]
[group("lint")]
deny:
    {{ cargo }} deny --all-features check

[doc("Run all tests.")]
[group("test")]
test package=build_package:
    {{ cargo }} nextest run \
        {{ if package == "" { "--workspace" } else { "-p " + package } }} \
        --all-features \
        --all-targets

[doc("Run crate documentation tests.")]
[group("docs")]
[group("test")]
test-docs package=build_package:
    {{ cargo }} test {{ if package == "" { "--all" } else { "-p " + package } }} --doc

[doc("Run coverage analysis.")]
[group("test")]
coverage package=build_package:
    {{ cargo }} tarpaulin \
        --all-targets \
        --all-features \
        --exclude-files **/examples/*.rs \
        {{ if package == "" { "--workspace " } else { "-p " + package } }}

[doc("Build Rust crates documentation")]
[group("docs")]
build-rustdocs package=build_package:
    RUSTDOCFLAGS="$RUSTDOCFLAGS --enable-index-page -Zunstable-options --default-theme=ayu" {{ cargo }} +nightly doc \
       {{ if package == "" { "--all" } else { "-p " + package } }} \
       --workspace \
       --no-deps \
       --document-private-items

[doc("Serve rustdocs.")]
[group("docs")]
[working-directory(".")]
@serve-rustdocs package=build_package:
    echo "Serving rustdocs... "
    {{ xdg-open }} {{ ROOT_DIR }}/target/doc/{{ package }}/index.html

[doc("Build project's documentation.")]
[group("docs")]
[working-directory("./xenith-website")]
@build-docs:
    echo "Building the documentation... "
    {{ hugo }} \
      --gc --minify

[doc("Serve the documentation.")]
[group("docs")]
[working-directory("./xenith-website")]
@serve-docs:
    echo "Serving the documentation... "
    {{ hugo }} server \
        --logLevel debug \
        --buildDrafts \
        --disableFastRender

[doc("Build project. Use release parameter for release builds. Use package parameter to build a specific package.")]
[group("build")]
build profile=build_profile package=build_package:
    {{ cargo }} build {{ if profile == "release" { "--release" } else { "" } }} --all-targets {{ if package == "" { "--workspace" } else { "-p " + package } }}

[doc("Run the project with configuration file. Use release parameter for release builds. Use package parameter to run a specific package.")]
[group("run")]
run profile=build_profile package=build_package: lint format
    {{ cargo }} run {{ if profile == "release" { "--release" } else { "" } }} {{ if package == "" { "" } else { "-p " + package } }}

[doc("Run a specific example with configuration file. Use release parameter for release builds.")]
[group("run")]
run-example example profile=build_profile:
    RUST_LOG=info {{ cargo }} run {{ if profile == "release" { "--release" } else { "" } }} --example {{ example }}

[group("quality")]
coupling package type="summary":
    #!/usr/bin/env bash
    set -euo pipefail

    case "{{ type }}" in
        summary)
            FLAG="--summary"
            ;;
        hotspots)
            FLAG="--hotspots"
            ;;
        web)
            FLAG="--web"
            ;;
        ai)
            FLAG="--ai"
            ;;
        *)
            echo "Error: You must use one of those: summary/hotspots/web/ai"
            exit 1
            ;;
    esac

    {{ cargo }} coupling "$FLAG" --exclude-tests {{ package }}
