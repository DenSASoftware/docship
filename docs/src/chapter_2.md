# Open help

To open the help use the `open-help` subcommand. This subcommand has this unusual name so it does not clash with the default `help` subcommand provided by the argparsing library.

```bash
docship open-help
```

This command will bind a http-server to port 10101 where it serves this documentation and open your browser at this location. To find out which program will be run look at [the opener crate](https://docs.rs/opener/0.4.1/opener) which uses its own version of [xdg-open](https://manpage.me/?q=xdg%2Dopen) on linux. If you don't want to open your browser automatically you can pass `--open-browser false` to the program.

