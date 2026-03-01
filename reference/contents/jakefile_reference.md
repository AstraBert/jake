---
intro: Jakefile Reference
tagline: How to write a Jakefile effectively
---

This is a Jakefile:

```toml
default = { command = "cat README.md" }
say-hello = "echo 'hello'"
say-hello-back = { command = "echo 'hello back'" }
say-bye = { command = "echo 'bye'", depends_on = ["say-hello", "say-hello-back"] }
list = "ls"
```
