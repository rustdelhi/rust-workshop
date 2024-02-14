---
marp: true
---

# Debugging

- VS Code supports Rust Debugging
- Basic debugging support is added by `rust-analyser`
- Create a `launch.json` file to enable debugging in your project.
- Example `launch.json`
    ```
        {
        // Use IntelliSense to learn about possible attributes.
        // Hover to view descriptions of existing attributes.
        // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
        "version": "0.2.0",
        "configurations": [
            {
                "type": "lldb",
                "request": "launch",
                "name": "Debug executable 'my_project'",
                "cargo": {
                    "args": [
                        "build",
                        "--bin=my_project",
                        "--package=my_project"
                    ],
                    "filter": {
                        "name": "my_project",
                        "kind": "bin"
                    }
                },
                "args": [],
                "cwd": "${workspaceFolder}"
            }
        ]
    }
    ```