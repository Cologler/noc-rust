# noc

*Run sub process without console.*

When I try schedule a task (like `pwsh -w Hidden -c ...`) to windows task scheduler,
the console window still flash up upon it start.

So...

## Usage

``` shell
noc pwsh -c '...'
```

Bye bye, console window.

### Features

- To quit the process, you **have to** use the task manager;
- No logs, unless you redirect the stdout and stderr to a file;
- Trance if the app try read char from stdin.

### More Use Cases

- deamon server like `mongod.exe` or `mariadbd.exe`, etc;
- `pueued` from https://github.com/Nukesor/pueue;
