### Installation  
> Install via cargo  
```shell
cargo install --locked yazi-fm yazi-cli
# open a new terminal and test installation
yazi
```

### Shell Wrapper  
> Provides the ability to change the current working directory when exiting Yazi.
>> Use `ra` to invoke Yazi.  
```shell
# For bash or zsh

function ra() {
	local tmp="$(mktemp -t "yazi-cwd.XXXXXX")" cwd
	yazi "$@" --cwd-file="$tmp"
	if cwd="$(command cat -- "$tmp")" && [ -n "$cwd" ] && [ "$cwd" != "$PWD" ]; then
		builtin cd -- "$cwd"
	fi
	rm -f -- "$tmp"
}

# For windows, Create the file ra.cmd and place it in your %PATH%.
# For Command Prompt

@echo off
set tmpfile=%TEMP%\yazi-cwd.%random%
yazi %* --cwd-file="%tmpfile%"
set /p cwd=<"%tmpfile%"
if not "%cwd%"=="" (
    cd /d "%cwd%"
)
del "%tmpfile%"
```

### Configuration  
> There are three configuration files for Yazi.  
- `yazi.toml`  
- `keymap.toml`  
- `theme.toml`  
> For Unix-like system, they should be placed at `~/.config/yazi/`  
> For Windows, `C:\Users\Username\AppData\Roming\yazi\config\` is the right place.  

#### Integrate with Neovim  
> For windows, environment variable `YAZI_FILE_ONE` for file is needed. 
>> For example `YAZI_FILE_ONE=C:\Program Files\Git\usr\bin\file.exe`  
```toml
# In yazi.toml  

[opener]
edit = [
	{ run = "nvim %*",  block = true, desc = "nvim", for = "windows" },
]
```

### Plugins  

#### BookMarks  
> Use [yamb.yazi](https://github.com/h-hg/yamb.yazi)  
```shell
# Install plugin via this cmd
ya pack -a h-hg/yamb
```

```lua
-- create init.lua to the directory holds yazi.toml and add this below
-- You can configure your bookmarks by lua language
local bookmarks = {}

local path_sep = package.config:sub(1, 1)
local home_path = ya.target_family() == "windows" and os.getenv("USERPROFILE") or os.getenv("HOME")
if ya.target_family() == "windows" then
  table.insert(bookmarks, {
    tag = "Scoop Local",
    
    path = (os.getenv("SCOOP") or home_path .. "\\scoop") .. "\\",
    key = "p"
  })
  table.insert(bookmarks, {
    tag = "Scoop Global",
    path = (os.getenv("SCOOP_GLOBAL") or "C:\\ProgramData\\scoop") .. "\\",
    key = "P"
  })
end
table.insert(bookmarks, {
  tag = "Desktop",
  path = home_path .. path_sep .. "Desktop" .. path_sep,
  key = "d"
})

require("yamb"):setup {
  -- Optional, the path ending with path seperator represents folder.
  bookmarks = bookmarks,
  -- Optional, recieve notification everytime you jump.
  jump_notify = true,
  -- Optional, the cli of fzf.
  cli = "fzf",
  -- Optional, a string used for randomly generating keys, where the preceding characters have higher priority.
  keys = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
  -- Optional, the path of bookmarks
  path = (ya.target_family() == "windows" and os.getenv("APPDATA") .. "\\yazi\\config\\bookmark") or
        (os.getenv("HOME") .. "/.config/yazi/bookmark"),
}

```
Add this to your `keymap.toml`  
```toml
[[manager.prepend_keymap]]
on = [ "u", "a" ]
run = "plugin yamb --args=save"
desc = "Add bookmark"

[[manager.prepend_keymap]]
on = [ "u", "g" ]
run = "plugin yamb --args=jump_by_key"
desc = "Jump bookmark by key"

[[manager.prepend_keymap]]
on = [ "u", "G" ]
run = "plugin yamb --args=jump_by_fzf"
desc = "Jump bookmark by fzf"

[[manager.prepend_keymap]]
on = [ "u", "d" ]
run = "plugin yamb --args=delete_by_key"
desc = "Delete bookmark by key"

[[manager.prepend_keymap]]
on = [ "u", "D" ]
run = "plugin yamb --args=delete_by_fzf"
desc = "Delete bookmark by fzf"

[[manager.prepend_keymap]]
on = [ "u", "A" ]
run = "plugin yamb --args=delete_all"
desc = "Delete all bookmarks"

[[manager.prepend_keymap]]
on = [ "u", "r" ]
run = "plugin yamb --args=rename_by_key"
desc = "Rename bookmark by key"

[[manager.prepend_keymap]]
on = [ "u", "R" ]
run = "plugin yamb --args=rename_by_fzf"
desc = "Rename bookmark by fzf"
```






