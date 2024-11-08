require("mason")setup({
    ui = {
        icons = {
            package_installed = "@",
            package_pending = "→",
            package_uninstalled "x"
        }
    }
})

require("mason-lspconfig").setup({
    ensure_installed = {
        "sumneko_lua",
    }
})
