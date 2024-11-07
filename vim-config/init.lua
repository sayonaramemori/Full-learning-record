-- Line numbers
vim.opt.relativenumber = true
vim.opt.number = true
-- vim.opt.cursorline = true  -- Uncomment if you want to enable cursorline
vim.opt.hlsearch = false

-- Encoding and History
vim.opt.encoding = "UTF-8"
vim.opt.history = 50  -- Uncomment if you want to set history length

-- Tabs and Indentation
vim.opt.expandtab = true
vim.opt.tabstop = 4
vim.opt.smarttab = true
vim.opt.shiftwidth = 4
vim.opt.softtabstop = 4

-- Command Bar
-- vim.opt.laststatus = 2  -- Uncomment if you want a constant status line
vim.opt.cmdheight = 2
vim.opt.autochdir = true
vim.opt.showcmd = true
vim.opt.showmode = true
vim.opt.ruler = true

-- Menus and Indentation
vim.opt.wildmenu = true
vim.opt.autoindent = true
vim.opt.smartindent = true

-- Display Characters and Scrolling
vim.opt.list = true
vim.opt.listchars = { tab = "▸ ", trail = "▫" }
vim.opt.scrolloff = 5

-- Timing
vim.opt.timeout = true
vim.opt.timeoutlen = 300
vim.opt.ttimeoutlen = 100

-- Syntax and Filetype
vim.cmd("syntax on")
vim.cmd("filetype plugin indent on")


-- Resize splits with arrow keys
vim.keymap.set("n", "<up>", ":vertical resize +5<CR>")
vim.keymap.set("n", "<down>", ":vertical resize -5<CR>")

-- Copy to system clipboard
vim.keymap.set("v", "Y", ":w !xclip -i -sel c<CR>")

-- Set <leader> as ;
vim.g.mapleader = ";"

-- Map `<Ctrl-q>` to visual block selection (`<Ctrl-v>`)
vim.keymap.set("v", "<C-q>", "<C-v>")

-- Open .vimrc
vim.keymap.set("n", "K", "o<Esc>")
vim.keymap.set("i", "<leader><leader>", "<Esc>")

-- Tab management
vim.keymap.set("n", "<leader>t", ":tabe<CR>")
vim.keymap.set("n", "<left>", ":-tabnext<CR>")
vim.keymap.set("n", "<right>", ":+tabnext<CR>")

-- Move between splits
vim.keymap.set("n", "<leader>k", "<C-w>k")
vim.keymap.set("n", "<leader>j", "<C-w>j")
vim.keymap.set("n", "<leader>h", "<C-w>h")
vim.keymap.set("n", "<leader>l", "<C-w>l")

-- Save and quit
vim.keymap.set("n", "<leader>w", ":wq<CR>")

-- Operating Vim on Windows
vim.keymap.set("n", "<leader>v", "<C-v>")
vim.keymap.set("n", "a", "A")

-- Plugin and utility commands
vim.keymap.set("n", "<leader>ta", ":TagbarToggle<CR>")
vim.keymap.set("n", "<leader><leader>", ":source ~/.vimrc<CR>")
vim.keymap.set("i", "{", "{<CR><CR>}<ESC>kcc")
vim.keymap.set("i", "[", "[]<ESC>i")

-- Additional mappings
vim.keymap.set("n", "-", "0")
vim.keymap.set("n", "=", "$")
vim.keymap.set("n", "<backspace>", "db")


-- Bootstrap lazy.nvim
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not (vim.uv or vim.loop).fs_stat(lazypath) then
  local lazyrepo = "https://github.com/folke/lazy.nvim.git"
  local out = vim.fn.system({ "git", "clone", "--filter=blob:none", "--branch=stable", lazyrepo, lazypath })
  if vim.v.shell_error ~= 0 then
    vim.api.nvim_echo({
      { "Failed to clone lazy.nvim:\n", "ErrorMsg" },
      { out, "WarningMsg" },
      { "\nPress any key to exit..." },
    }, true, {})
    vim.fn.getchar()
    os.exit(1)
  end
end
vim.opt.rtp:prepend(lazypath)

-- Make sure to setup `mapleader` and `maplocalleader` before
-- loading lazy.nvim so that mappings are correct.
-- This is also a good place to setup other settings (vim.opt)
-- vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

-- Setup lazy.nvim
require("lazy").setup({
  spec = {
    -- add your plugins here
    {
        "iamcco/markdown-preview.nvim",
        cmd = { "MarkdownPreviewToggle", "MarkdownPreview", "MarkdownPreviewStop" },
        ft = { "markdown" },
        build = function() vim.fn["mkdp#util#install"]() end,
    },
    {
        "vim-airline/vim-airline"
    },
    {
        "rafi/awesome-vim-colorschemes"
    },
    {
        'connorholyday/vim-snazzy'
    },
    {
        "rebelot/kanagawa.nvim"
    },
    {
        "scottmckendry/cyberdream.nvim",
        lazy = false,
        priority = 1000,
    },
    {
        "octol/vim-cpp-enhanced-highlight"
    }
  },
  -- Configure any other settings here. See the documentation for more details.
  -- colorscheme that will be used when installing plugins.
  install = { colorscheme = { "habamax" } },
  -- automatically check for plugin updates
  checker = { enabled = true },
})

require("cyberdream").setup({
    -- Enable transparent background
    transparent = true,

    -- Enable italics comments
    italic_comments = false,

    -- Replace all fillchars with ' ' for the ultimate clean look
    hide_fillchars = false,

    -- Modern borderless telescope theme - also applies to fzf-lua
    borderless_telescope = true,

    -- Set terminal colors used in `:terminal`
    terminal_colors = true,

    -- Improve start up time by caching highlights. Generate cache with :CyberdreamBuildCache and clear with :CyberdreamClearCache
    cache = false,

    theme = {
        variant = "default", -- use "light" for the light variant. Also accepts "auto" to set dark or light colors based on the current value of `vim.o.background`
        saturation = 1, -- accepts a value between 0 and 1. 0 will be fully desaturated (greyscale) and 1 will be the full color (default)
        highlights = {
            -- Highlight groups to override, adding new groups is also possible
            -- See `:h highlight-groups` for a list of highlight groups or run `:hi` to see all groups and their current values

            -- Example:
            Comment = { fg = "#696969", bg = "NONE", italic = true },

            -- Complete list can be found in `lua/cyberdream/theme.lua`
        },

        -- Override a highlight group entirely using the color palette
        overrides = function(colors) -- NOTE: This function nullifies the `highlights` option
            -- Example:
            return {
                Comment = { fg = colors.green, bg = "NONE", italic = true },
                ["@property"] = { fg = colors.magenta, bold = true },
            }
        end,

        -- Override a color entirely
        colors = {
            -- For a list of colors see `lua/cyberdream/colours.lua`
            -- Example:
            bg = "#000000",
            green = "#00ff00",
            magenta = "#ff00ff",
        },
    },

    -- Disable or enable colorscheme extensions
    extensions = {
        telescope = true,
        notify = true,
        mini = true,
        ...
    },
})

vim.cmd("colorscheme cyberdream")

vim.g.cpp_class_scope_highlight = true
