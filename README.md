# NeoInked - VS Code Dark Theme

NeoInked is a dark theme for Visual Studio Code that is designed to be easy on the eyes and provide an enjoyable coding experience. 
It features a carefully crafted color scheme with subtle shades that make syntax highlighting easy to read.

## Preview

Here's a preview of the NeoInked theme:

![NeoInked Theme Preview](images/file_cpp_preview.png)
NeoInked currently has been tested on : C/C++, Rust, Javascript, Python, HTML, CSS, and plain text files.

## Installation

To install NeoInked, follow these steps:

1. Open the Extensions view in VS Code by clicking on the Extensions icon in the Activity Bar on the side of the window or by pressing `Ctrl+Shift+X` (`Cmd+Shift+X` on a Mac).
2. Search for "NeoInked" in the search bar.
3. Click the "Install" button to install the extension.

<!-- Alternatively, you can download the extension from the [Visual Studio Code Marketplace](https://marketplace.visualstudio.com/items?itemName=your-username.NeoInked) and install it manually. -->



## Customization

If you want to customize the theme, you can do so by creating a `settings.json` file in your `.vscode` folder and adding the following code:

```json
{
  "workbench.colorTheme": "NeoInked",
  "editor.tokenColorCustomizations": {
    "comments": "#888888",
    "strings": "#ff0000",
    "numbers": "#008000"
  }
}
```

## Problems/Issues
If you encounter a problem or issue feel free to open a new issue on my [github](https://github.com/Lamperr/NeoInked/issues) 