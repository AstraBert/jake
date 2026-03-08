---
intro: Reference
tagline: Install, write tasks and run them!
---

### Installation

With cargo (recommended):

```bash
cargo install jake
```

With npm:

```bash
npm install -g @cle-does-things/jake@latest
```

### Initialization

You can create a boilerplate `jakefile.toml` with the `--init` option:

```bash
jake --init "task1,task2" # tasks need to be provided as a comma-separated list
```

This will write a `jakefile.toml` in the current working directory, which will look like this:

```toml
task1 = "echo 'No task yet for task1'"
task2 = "echo 'No task yet for task2'"
```

You can then customize the file as needed.

### Task Definition

Tasks are defined in a file called `jakefile.toml` placed either in the working directory where `jake` is executed, or anywhere up the directory tree. Each entry in the file represents a task, mapping a task name to either a plain string command or an object with additional configuration.

A task can be defined in two ways:

**As a plain string**: use this when the task has no dependencies:

```toml
say-hello = "echo 'hello'"
list = "ls"
```

**As an object**: use this when you need to specify dependencies:

```toml
say-hello-back = { command = "echo 'hello back'" }
say-bye = { command = "echo 'bye'", depends_on = ["say-hello", "say-hello-back"] }
```

The anatomy of an object task is as follows:

```text
say-bye = { command = "echo 'bye'", depends_on = ["say-hello", "say-hello-back"] }
   |                |                            |
Task name    Command to execute    Array of tasks to be executed before
                                          the task itself
```

`command` is required when using the object syntax. `depends_on` is optional: if omitted, the task runs with no prerequisites.

### The Default Task

You can designate a task to run when no task name is passed to `jake` by naming it `default`:

```toml
default = { command = "cat README.md" }
```

If no `default` task is explicitly defined, `jake` will fall back to the first task in the file.

### Tasks Referencing Environment Variables

You can use environment variables inside any task command:

```toml
env_var = "echo $HELLO"
```

`jake` resolves environment variables from `export` statements or a `.env` file, either in the working directory where `jake` is executed, or anywhere up the directory tree.

To enable loading `.env` files, you need to provide the `--env` flag to the `jake` command.

### Full Example

```toml
default = { command = "cat README.md" }
say-hello = "echo 'hello'"
say-hello-back = { command = "echo 'hello back'" }
say-bye = { command = "echo 'bye'", depends_on = ["say-hello", "say-hello-back"] }
list = "ls"
```

### Running Tasks

`jake` can be invoked from any subdirectory of the project: it will walk up the directory tree to locate the nearest `jakefile.toml`.

**List all available tasks**

```bash
jake --list
```

**Execute the default task**

```bash
jake
```

**Execute a specific task**

```bash
jake say-hello
```
```text
'hello'
```

**Execute a task with dependencies**

When a task declares `depends_on`, all listed tasks are executed first, in order, before the task itself runs:

```bash
jake say-bye
```
```text
'hello'
'hello back'
'bye'
```

**Pass additional options to a task**

You can forward extra flags to the underlying command using `--options`:

```bash
jake list --options "-la"
```

This will output:

```text
total 48
drwxr-xr-x@  10 user  staff   320 Feb 13 11:14 .
drwxr-xr-x@ 125 user  staff  4000 Feb 13 10:20 ..
drwxr-xr-x@   9 user  staff   288 Feb 13 10:20 .git
-rw-r--r--@   1 user  staff     8 Feb 13 10:20 .gitignore
-rw-r--r--@   1 user  staff  7656 Feb 13 11:13 Cargo.lock
-rw-r--r--@   1 user  staff   162 Feb 13 11:13 Cargo.toml
-rw-r--r--@   1 user  staff   332 Feb 13 11:21 jakefile.toml
-rw-r--r--@   1 user  staff   152 Feb 13 11:16 README.md
drwxr-xr-x@   4 user  staff   128 Feb 13 10:22 src
drwxr-xr-x@   6 user  staff   192 Feb 13 10:22 target
```

The value passed to `--options` is appended to the task's command at execution time, so `jake list --options "-la"` effectively runs `ls -la`.

**Load a `.env` file and execute a task**

If a task requires an environment variable, e.g.:

```toml
env_var = "echo $HELLO"
```

You can either provide it with an `export` statement or define it within a `.env` file:

```env
HELLO="hello"
```

Now run `jake` with `--env`:

```bash
jake env_var --env
```

Output:

```text
hello
```
