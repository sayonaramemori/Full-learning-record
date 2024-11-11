require("mason").setup({
    ui = {
        icons = {
            package_installed = "@",
            package_pending = "→",
            package_uninstalled = "x",
        },
    },
})

require("mason-lspconfig").setup({
    ensure_installed = {
        "clangd",
    }
})

local capabilities = require('cmp_nvim_lsp').default_capabilities()

local lspconfig = require("lspconfig")

lspconfig.clangd.setup{
    capabilities = capabilities,
}


