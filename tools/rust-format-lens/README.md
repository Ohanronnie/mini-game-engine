# Rust Format Lens

Adds **Format** above Rust's `main` function alongside Rust Analyzer's Run and Debug actions. Clicking it formats the current file with VS Code's configured Rust formatter. Changes remain in the editor until saved.

Requires Rust Analyzer with CodeLens enabled.

To package and install after changing the extension:

```sh
cd tools/rust-format-lens
bunx @vscode/vsce@latest package --allow-missing-repository --skip-license --out /tmp/rust-format-lens.vsix
code --install-extension /tmp/rust-format-lens.vsix --force
```

Run **Developer: Reload Window** in VS Code after installing or updating.
