# jake

`jake` (crasis for "just make") is a [Make](https://en.wikipedia.org/wiki/Make_(software))-like task executor for Unix-based operating systems.

It is based on TOML tasks definition (stored in a file called `jakefile.toml`) and can execute commands resolving their dependencies and passing additional options.

## Features

- Simple TOML syntax for task definition (no .PHONY, no spacing rules)
- Dependency resolution with circular dependencies issues detection
- Allows to pass extra arguments (as options) from the command line
- Default command execution

## Installation

With cargo (recommended):

```bash
cargo install jake
```

With npm:

```bash
npm install -g @cle-does-things/jake@latest
```

## Example `jakefile.toml`

Here is an example for task definition:

```toml
default = { command = "cat README.md" }
say-hello = "echo 'hello'"
say-hello-back = { command = "echo 'hello back'" }
say-bye = { command = "echo 'bye'", depends_on = ["say-hello", "say-hello-back"] }
list = "ls"
```

And here is the anatomy of a definition:

```text
say-bye = { command = "echo 'bye'", depends_on = ["say-hello", "say-hello-back"] }
   |                |                            |
Task name    Command to execute    Array of tasks to be executed _before_
                                               the task itself
```

While `depends_on` is optional (if not provided, the task does not depend on anything), `command` is a required key if the provided TOML value is an object (as for `say-hello-back`, `default` and `say-bye`).

In cases where the command does not depend on anything, you can also provide it as a plain string (as for `say-hello` and `list`).

You can use the `default` task name to indicate the task that should be executed by default when none is provided to `jake` (otherwise, the default task will be the first one in the file).

## Example Usage

Execute default task:

```bash
jake
```

Output:

```text
# jake

`jake` (crasis for "just make") is a [Make](https://en.wikipedia.org/wiki/Make_(software))-like task executor for Unix-based operating systems.

...
```

Execute a task that does not depend on anything:

```bash
jake say-hello
```

Output:

```text
'hello'
```

Execute a task that depends on other tasks:

```bash
jake say-bye
```

Output:

```text
'hello'
'hello back'
'bye'
```

Execute a task passing additional flags:

```bash
jake list --options "-la"
```

Output:

```text
total 48
drwxr-xr-x@  10 clee  staff   320 Feb 13 11:14 .
drwxr-xr-x@ 125 clee  staff  4000 Feb 13 10:20 ..
drwxr-xr-x@   9 clee  staff   288 Feb 13 10:20 .git
-rw-r--r--@   1 clee  staff     8 Feb 13 10:20 .gitignore
-rw-r--r--@   1 clee  staff  7656 Feb 13 11:13 Cargo.lock
-rw-r--r--@   1 clee  staff   162 Feb 13 11:13 Cargo.toml
-rw-r--r--@   1 clee  staff   332 Feb 13 11:21 jakefile.toml
-rw-r--r--@   1 clee  staff   152 Feb 13 11:16 README.md
drwxr-xr-x@   4 clee  staff   128 Feb 13 10:22 src
drwxr-xr-x@   6 clee  staff   192 Feb 13 10:22 target
```

## In GitHub CI/CD

You can use the [setup-jake](https://github.com/AstraBert/setup-jake) action to set up jake and use it in your GitHub Actions workflows.

## License

This project is provided under MIT license.
