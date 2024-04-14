use glob::glob;

pub fn find_modules(pattern: &str) -> Result<Vec<String>, glob::GlobError> {
    let mut modules = Vec::new();

    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    modules.push(path.to_str().unwrap().to_string());
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(modules)
}

pub fn lua_module_name(path: &str, prefix: Option<&str>) -> String {
    let mut parts = path.split(std::path::MAIN_SEPARATOR).collect::<Vec<_>>();

    if let Some(p) = prefix {
        if !p.is_empty() {
            parts.insert(0, p);
        } else if parts.len() > 1 {
            parts.remove(0);
        }
    }

    parts.join(".").replace(".lua", "")
}

#[test]
fn test_find_modules() {
    let modules = find_modules("lib/**/*.lua");

    println!("{:#?}", modules);

    println!("{:#?}", lua_module_name("lib\\util.lua", Some("util")));
}