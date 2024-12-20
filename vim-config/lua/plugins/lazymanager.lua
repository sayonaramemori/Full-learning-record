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
          build = "cd app && npm install",
          init = function()
            vim.g.mkdp_filetypes = { "markdown" }
          end,
          ft = { "markdown" },
    },
    {
        "nvim-tree/nvim-tree.lua",
         version = "*",
         lazy = false,
         dependencies = {
           "nvim-tree/nvim-web-devicons",
         },
--         config = function()
--            require("nvim-tree").setup {}
--         end,
    },
    {
        "octol/vim-cpp-enhanced-highlight"
    },
    {
        "nvim-lualine/lualine.nvim",
         dependencies = { 'nvim-tree/nvim-web-devicons' }
    },
    {
         "scottmckendry/cyberdream.nvim",
         lazy = false,
         priority = 1000,
    },
    {   
         'akinsho/bufferline.nvim', 
         version = "*", 
         dependencies = 'nvim-tree/nvim-web-devicons',
    },
    {
        "nvim-treesitter/nvim-treesitter",
    },
    {
        "p00f/nvim-ts-rainbow",
    },
    --[[ {
        "williamboman/mason.nvim",
        "williamboman/mason-lspconfig.nvim",
        "neovim/nvim-lspconfig",
    }, ]]
    {
        'numToStr/Comment.nvim',
        opts = {
            -- add any options here
        }
    },
    {
        "lewis6991/gitsigns.nvim",
    }
  },
  -- Configure any other settings here. See the documentation for more details.
  -- colorscheme that will be used when installing plugins.
  install = { colorscheme = { "habamax" } },
  -- automatically check for plugin updates
  checker = { enabled = true },
})

