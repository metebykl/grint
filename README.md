<div align="center">
  <h1><code>grint</code></h1>
</div>

`grint` is a modern, declarative command runner.

## Installation

TODO

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
