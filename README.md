<div align="center">
  <h1><code>grint</code></h1>
</div>

`grint` is a modern, declarative command runner.

## Installation

### Prerequisites

`grint` runs on any system with a Unix-like shell, including Linux and MacOS.

#### Windows

On Windows, `grint` uses PowerShell by default.

If you'd prefer not use Powershell, `grint` also works with the `sh`
provided by [Git Bash](https://git-scm.com), [Cygwin](https://www.cygwin.com)
or [WSL](https://learn.microsoft.com/en-us/windows/wsl).

If you're using Git Bash, Cygwin, or another POSIX-like environment on Windows
and want to run your tasks using `sh`, you must explicitly set the shell
with `--shell sh --shell-arg -cu`.

You can also configure `grint` to use any shell using command-line arguments. For example,
to use the windows command prompt, launch `grint` with `--shell cmd.exe --shell-arg /C`.

(PowerShell is the recommended shell for Windows users.)

## Quick Start

See the [installation section](#installation) for how to install `grint` on your computer.

Once `grint` is installed, create a file named `Grint.toml` in the root
of your project with the following contents:

```toml
[task.example]
cmd = "This is an example task!"
```

When you invoke `grint` it looks for a `Grint.toml` file in the current directory.

Run `grint` by specifying the task to run:

```sh
grint example
```
