<div align="center">
  <h1><code>grint</code></h1>
</div>

`grint` is a modern, declarative command runner.

## Installation

### Prerequisites

`grint` runs on any system with a unix shell, including Linux and MacOS.

#### Windows

On windows `grint` uses PowerShell by default.

If you'd rather not use Powershell, `grint` also works with the `sh`
provided by [Git for Windows](https://git-scm.com) or [Cygwin](https://www.cygwin.com).

You can also use any shell you want using command-line arguments. For example,
to use the windows command prompt, launch `grint` with `--shell cmd.exe --shell-arg /C`.

(Using PowerShell is recommended for Windows users.)

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
