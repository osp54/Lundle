## Lundle
Lundle is a package bundler that allows you to easily bundle your lua files into a single file.
Configuration is done through a buildscript file, which is a lua file that exports a table with the configuration, and can post-process the bundled file(for example, to add a shebang line, minify).

## Installation
obtain the latest release from the releases page, or build from source.

## Building from source
```bash
git clone
cd Lundle

cargo build
```

## Usage
For building a project, you need to create a buildscript file (default name is `lundle.build.lua`), and run `lundle` in the same directory as the buildscript file.

Parameters to the `lundle` command:
- `-b`, `--buildscript`: The buildscript file to use, defaults to `lundle.build.lua`

### Example buildscript
```lua
local c = config

c.main = "main.lua"
c.encoding = "utf-8"

c.output = "mainbundled.lua"

c:add_module("lib/*.lua")
```
There are a few configuration options:
- `main`: The entry point of the program
- `encoding`: The encoding of the files, defaults to utf-8
- `output`: The output file
- `add_module`: Adds a module to the bundle, can be a single file or a glob pattern. In the example above, all lua files in the `lib` directory will be added to the bundle.

### Post-processing
You can post-process the bundled file by adding a `postprocess` function to the buildscript file.
```lua
local c = config
...
function postprocess(content)
    return os.date("-- Built on %Y-%m-%d %H:%M:%S\n") .. content
end
```
The `postprocess` function takes the bundled file content as an argument, and should return the processed content.
In the example above, the bundled file will have a line added at the top with the current date and time (shebang line).

# API Reference
## Config
### `config.main`
The entry point of the program, defaults to `main.lua`
### `config.encoding`
The encoding of the files, defaults to `utf-8`
### `config.output`
The output file, defaults to `main.bundled.lua`
### `config:add_module(path: string, [prefix: string])`
Adds a module to the bundle, can be a single file or a glob pattern.
- `path`: The path to the module
- `prefix`: The prefix to add to the module name in the bundle. If was empty string, the first part of the path will be removed. If not empty, the prefix will be added to first part of the path.
### `config:remove_module(path: string)`
Removes a module from the bundle. Only works if the module was added with `add_module`.
- `path`: The path to the module

## License
This project is licensed under the MIT License.

## Credits
- [rust](https://www.rust-lang.org/)
- [clap](https://github.com/clap-rs/clap)
- [mlua](https://github.com/mlua-rs/mlua)
