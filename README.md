<div align="center">
  <h1><code>grint</code></h1>
</div>

`grint` is a modern, declarative command runner.

## Installation

### Prerequisites

`grint` runs on any system with a Unix-like shell, including Linux and MacOS.

#### Windows

On Windows, `grint` uses PowerShell by default.

If you'd prefer not to use Powershell, `grint` also works with the `sh`
provided by [Git Bash](https://git-scm.com), [Cygwin](https://www.cygwin.com)
or [WSL](https://learn.microsoft.com/en-us/windows/wsl).

If you're using Git Bash, Cygwin, or another POSIX-like environment on Windows
and want to run your tasks using `sh`, you must explicitly set the shell
with `--shell sh --shell-arg -cu`.

You can also configure `grint` to use any shell using command-line arguments.
For example, to use the Windows command prompt, launch `grint`
with `--shell cmd.exe --shell-arg /C`.

(PowerShell is the recommended shell for Windows users.)

## Quick Start

See the [installation section](#installation) for how to install `grint` on
your computer. Make sure that your installation is correct by running
`grint --version`.

Once `grint` is installed, create a file named `Grint.toml` in the root
of your project with the following contents:

```toml
[task.example]
cmd = "echo 'This is an example task!'"
```

When you invoke `grint` it looks for a `Grint.toml` file in the current
directory.

Run `grint` by specifying the task to run:

```console
$ grint example
> echo 'This is an example task!'
This is an example task!
```

## Features

- [Listing Available Tasks](#listing-available-tasks)
- [Working Directory](#working-directory)
- [Task Descriptions](#task-descriptions)
- [Dependencies](#dependencies)
- [Environment Variables](#environment-variables)

### Listing Available Tasks

Tasks can be listed with `grint --list`:

```console
$ grint --list
Available tasks:
  lint
  test
  build
```

### Working Directory

By default, tasks run with the working directory that contains the
`Grint.toml` file.

You can override the working directory for specific tasks using the `cwd`
attribute.

```toml
[task.foo]
cwd = "bar"
cmd = "pwd"
```

```console
$ pwd
/home/foo
$ grint foo
> pwd
/home/foo/bar
```

### Task Descriptions

Task descriptions will appear in `grint --list`:

```toml
[task.build]
desc = "build app"
cmd = "./bin/build"

[task.test]
desc = "test app"
cmd = "./bin/test"
```

```console
$ grint --list
Available tasks:
  build  # build app
  test   # test app
```

### Dependencies

Dependencies run before tasks that depend on them:

```toml
[task.build]
cmd = "cc main.c"

[task.test]
deps = ["build"]
cmd = "./a.out --test"
```

```console
$ grint test
> cc main.c
> ./a.out --test
```

### Environment Variables

You can set environment variables for specific tasks using the `env`
attribute.

```toml
[task.name]
env = { name = "grint" }
cmd = "echo $name"
```

```console
$ grint name
> echo $name
grint
```

[🔼 Back to the top](#grint)
