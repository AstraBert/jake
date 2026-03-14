# jake

`jake` (crasis for "just make") is a [Make](https://en.wikipedia.org/wiki/Make_(software))-like task executor for Unix-based operating systems.

It is based on TOML tasks definition (stored in a file called `jakefile.toml`) and can execute commands resolving their dependencies and passing additional options.

## Features

- Create a boilerplate `jakefile.toml` file with `jake --init 'task1,task2,...'`
- Simple TOML syntax for task definition (no .PHONY, no spacing rules)
- Dependency resolution with circular dependencies issues detection
- Allows to pass extra arguments (as options) from the command line
- Default command execution
- Evaluates composite commands (like `cat README.md | grep Features` or `cd src/ && pwd`)
- You can execute a task from any subdirectory of the directory where `jakefile.toml` is stored
- You can list tasks, by passing the `--list` flag
- You can load `.env` file (in the same working directory or anywhere up in the directory tree), by passing the `--env` flag
- Execute scripts from a `package.json` file with the `--js` flag.

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
env-variable = "echo $HELLO"
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

Create a boilerplate `jakefile.toml`:

```bash
jake --init 'task1,task2' # tasks need to be provided as a comma-separated list
```

Output (as `jakefile.toml` in the current working directory):

```toml
task1 = "echo 'No task yet for task1'"
task2 = "echo 'No task yet for task2'"
```

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

List all tasks:

```bash
jake --list
```

Output:

```text
Available tasks:
- default
- say-hello
- say-hello-back
...
```

Run a task that requires an environment variable by loading a `.env` file:

```bash
echo "HELLO=hello" >> .env
jake env-var --env
```

Output:

```text
hello
```

If you are in a JS/TS application using a `package.json` for its scripts, as in this example:

```json
{
  "scripts": {
    "test": "vitest",
    "dev": "vite"
  }
}
```

You can invoke the scripts with `jake` using the `--js` flag:

```bash
jake dev --js
```

You can invoke a script from the directory in which `package.json` is stored and from all its subdirectories.

## In GitHub CI/CD

You can use the [setup-jake](https://github.com/AstraBert/setup-jake) action to set up jake and use it in your GitHub Actions workflows.

## License

This project is provided under MIT license.
