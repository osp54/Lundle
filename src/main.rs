mod util;
mod squisher;

use crate::util::baseconfig::Config;
use crate::util::luamodule::find_modules;
use crate::squisher::squish_module;

use clap::Parser;
use encoding_rs::Encoding;
use mlua::prelude::*;
use util::files::{read_file_in_defined_encoding, read_file_in_encoding, write_file_in_encoding};

#[derive(Parser, Debug)]
#[command(name = "Lundle")]
#[command(version = "0.1", about = "A module bundler for Lua.")]
#[command(long_about = None)]

struct Args {
    #[arg(short, long, default_value_t = String::from("lundle.build.lua"))]
    buildscript: String,
}

fn main() -> LuaResult<()> {
    let args = Args::parse();

    let lua = Lua::new();
   
    lua.globals().set("config", Config::new())?;

    let bs_content = std::fs::read_to_string(&args.buildscript)
        .or_else(|err| {
            if err.kind() == std::io::ErrorKind::InvalidData {
                read_file_in_defined_encoding(&args.buildscript)
            } else {
                Err(err)
            }
    }).expect("Failed to read buildscript");

    lua.load(&bs_content)
        .set_name(&args.buildscript)
        .exec()?;

    let config: Config = lua.globals().get("config").expect("Invalid config");
    let encoding: &Encoding = Encoding::for_label(config.encoding.as_bytes())
        .expect("Invalid main file encoding");

    let mut result: String = String::new();
    for module in config.modules {
        let found_modules = find_modules(module[0].as_str()).expect("Failed to find modules");

        if found_modules.is_empty() {
            println!("No modules found for pattern: {}", module[0]);
            continue;
        }

        for found_module in found_modules {
            println!("Squishing module: {}", found_module);
            let prefix = module.get(1).map(|p| p.as_str());

            let squished = squish_module(&found_module, prefix, encoding);
            result.push_str(&squished);
        }
    }

    let main_file_content = read_file_in_encoding(&config.main, encoding)
        .expect("Failed to read main file");
    result.push_str(&main_file_content);

    let output = config.output;

    if let Ok(func) = lua.globals().get::<_, LuaFunction>("postprocess") {
        let lres = func.call::<String, String>(result.clone())?;

        result = lres;
    }

    write_file_in_encoding(&output, &result, encoding)
        .expect("Failed to write output");

    println!("Output written to: {}, {} bytes", output, result.len());
        
    Ok(())
}