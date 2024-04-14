use crate::util::luamodule::lua_module_name;
use crate::util::files::read_file_in_encoding;

pub fn squish_module(module: &str, prefix: Option<&str>, encoding: &'static encoding_rs::Encoding) -> String {
    let contents = read_file_in_encoding(module, encoding).expect("Failed to read module");
    let lua_module = lua_module_name(module, prefix);

    format!("package.preload[\"{}\"] = (function(...)\n{}\nend)\n", lua_module, contents)
}