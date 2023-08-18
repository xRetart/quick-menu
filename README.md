# quick-menu
`quick-menu` is a terminal based menu application.
It reads the possible options in a specified format from `stdin`.
The menu is displayed in form of a list over `stderr` and the user is able to choose an option by scrolling or pressing the hotkey associated with an option.
If an option was chosen, it is printed to `stdout` and the program terminates.

# installation
1. build the binary
```sh
$ cargo build --release
```

2. install the binary to its destination (in this case ~/.local/bin)
```sh
$ mv target/release/quick-menu "$HOME"/.local/bin
```

# usage
To provide `quick-menu` with the desired options you have to write the options in the following format to `stdin`.
```quick-menu
<hotkey> : <value>|<displayed>
```
where `displayed` is the string listed in the menu and `value` the string outputted when the option is chosen.  
Currently command line arguments are ignored.
