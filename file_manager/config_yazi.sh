# echo "Installing Yazi, Rust Environment is needed" &&
# cargo install --locked yazi-fm yazi-cli &&
# mkdir ~/.config/yazi -p  &&
# cp ./keymap.toml ~/.config/yazi/keymap.toml &&
# cp ./yazi.toml   ~/.config/yazi/yazi.toml &&
# cp ./init.lua    ~/.config/yazi/init.lua &&
# ya pack -a yazi-rs/plugins:full-border &&
# ya pack -a h-hg/yamb &&
cat << 'EOF' >> ~/.bashrc
function ra() {
	local tmp="$(mktemp -t "yazi-cwd.XXXXXX")" cwd
	yazi "$@" --cwd-file="$tmp"
	if cwd="$(command cat -- "$tmp")" && [ -n "$cwd" ] && [ "$cwd" != "$PWD" ]; then
		builtin cd -- "$cwd"
	fi
	rm -f -- "$tmp"
} 
EOF

source ~/.bashrc


