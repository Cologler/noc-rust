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
