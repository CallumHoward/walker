## About
This simple utility was built to be used with [skim](https://github.com/lotabout/skim) (or [fzf](https://github.com/junegunn/fzf)).
With the following Zsh function, it works as a fuzzy cd.

```zsh
# cf - fuzzy cd
function cf() {
    local bin=~/.cargo/bin/exa
    if ! [ -x $bin ]; then bin = ls; fi
    if [ "$1" != "" ]; then cd $1; fi
    local dir=`walker | rg -v "/\.|\.dSYM|\.xcodeproj|build|bin|ctc|^$" | sk`
    if [ "$dir" != "" ]; then cd $dir; fi
    # ls after cd, but only if there are less then 50 items in the directory
    if (( `ls | wc -l` < 50 )); then $bin --group-directories-first -x; fi
}
```

Using this utility is much faster for this than using `find -type d`. Directories are walked in parallel, and it works fast enough to be usable from my home directory.

## Dependencies
Dependencies for the above zsh function:  
- [skim](https://github.com/lotabout/skim)  
- [ripgrep](https://github.com/BurntSushi/ripgrep)  
- [exa](https://github.com/ogham/exa) (optional)  


## Install
Build from source with `cargo build --release`.  
Copy binary into a location in your path.
