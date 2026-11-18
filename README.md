<pre>
┏━┓┏━┓╺┳╸╻ ╻   ┏━┓╻ ╻┏━┓┏━╸┏━╸
┣━┛┣━┫ ┃ ┣━┫   ┣━┛┃ ┃┣┳┛┃╺┓┣⸸ 
╹  ╹ ╹ ╹ ╹ ╹   ╹  ┗━┛╹┗╸┗━┛┗━╸
</pre>

`pathpurge` is a minimalist, high-performance command-line utility written in Rust to audit, analyze, and clean up your system `$PATH` environment variable.

## Features

- **Cross-Platform Host Detection**: Automatically detects your operating system, architecture, and active shell.
- **Interactive Auditing**: Inspects every directory in your `$PATH`, catching duplicates, missing directories, and invalid file paths.
- **Interactive Resolution**: Prompts you item-by-item to [k]eep or [d]elete (`[k]/[d]`) problematic paths.
- **Clean Export Generation**: Outputs a ready-to-use, deduplicated `export PATH="..."` string at the end of the audit.
- **Zero Dependencies Bloat**: Built with native Rust collections and `clap` for clean, predictable CLI behavior.

---

## Installation

Ensure you have the [Rust toolchain](https://www.rust-lang.org/) installed on your system.

1. Clone the repository:
   ```bash
   git clone [https://github.com/diegoolivaa/pathpurge.git](https://github.com/diegoolivaa/pathpurge.git)
   cd pathpurge
