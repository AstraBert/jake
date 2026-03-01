---
intro: Features
tagline: Jake features and comparison
---

`jake` (crasis for "just make") is a [Make](https://en.wikipedia.org/wiki/Make_(software))-like task executor for Unix-based operating systems.
It is based on TOML task definitions (stored in a file called `jakefile.toml`) and can execute commands by resolving their dependencies and forwarding additional options from the command line.

### Features

- **Simple TOML syntax** for task definition: no `.PHONY` declarations, no spacing rules
- **Dependency resolution** with circular dependency detection
- **Extra arguments** can be passed as options directly from the command line
- **Default task execution** when no task name is specified
- **Composite commands** support (e.g. `cat README.md | grep Features` or `cd src/ && pwd`)
- **Subdirectory awareness**: tasks can be invoked from any subdirectory of the directory containing `jakefile.toml`

### Comparison

The table below compares `jake` against [`just`](https://github.com/casey/just) and [`make`](https://en.wikipedia.org/wiki/Make_(software)) across key features:

| Feature                          | jake | just | make |
|----------------------------------|------|------|------|
| Dependency graph resolution      | ✅   | ❌    | ✅  |
| Circular dependency detection    | ✅   | ❌    | ❌   |
| Extra arguments / options        | ✅   | ✅   | ⚠️  |
| Default task execution           | ✅   | ✅   | ✅  |
| Composite commands               | ✅   | ✅   | ✅  |
| Subdirectory invocation          | ✅   | ✅   | ❌   |
| Simple, readable syntax          | ✅   | ✅   | ❌   |
| No special spacing rules         | ✅   | ✅   | ❌   |
| Read .env                        | ❌    | ✅   | ❌   |
| List available commands          | ❌    | ✅   | ❌   |
| Recipes written in arbitrary languages | ❌ | ✅   | ❌   |


⚠️ `make` supports passing variables from the command line but not named options in the same ergonomic way.
