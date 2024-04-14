use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, mlua::FromLua)]
pub struct Config {
    pub main: String,
    pub encoding: String,
    pub output: String,
    pub modules: Vec<Box<[String]>>,

    pub minify: bool,
    pub preserve_comments: bool
}

impl Config {
    pub fn new() -> Self {
        Self {
            main: String::from("main.lua"),
            encoding: String::from("utf-8"),

            output: String::from("main.bundle.lua"),
            modules: Vec::new(),

            minify: false,
            preserve_comments: false
        }
    }

    pub fn push_module(&mut self, module: String, prefix: Option<String>) {
        self.modules.push({
            if let Some(p) = prefix {
                Box::new([module, p])
            } else {
                Box::new([module])
            }
        });
    }

    pub fn remove_module(&mut self, module: String) {
        self.modules.retain(|m| m[0] != module);
    }
}

impl mlua::UserData for Config {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("main", |_, this| Ok(this.main.clone()));
        fields.add_field_method_get("encoding", |_, this| Ok(this.encoding.clone()));
        fields.add_field_method_get("output", |_, this| Ok(this.output.clone()));
        fields.add_field_method_get("modules", |_, this| Ok(this.modules.clone()));

        fields.add_field_method_get("minify", |_, this| Ok(this.minify));
        fields.add_field_method_get("preserve_comments", |_, this| Ok(this.preserve_comments));


        fields.add_field_method_set("main", |_, this, value| {
            this.main = value;
            Ok(())
        });
        fields.add_field_method_set("encoding", |_, this, value| {
            this.encoding = value;
            Ok(())
        });
        fields.add_field_method_set("output", |_, this, value| {
            this.output = value;
            Ok(())
        });
        fields.add_field_method_set("modules", |_, this, value| {
            this.modules = value;
            Ok(())
        });

        fields.add_field_method_set("minify", |_, this, value| {
            this.minify = value;
            Ok(())
        });

        fields.add_field_method_set("preserve_comments", |_, this, value| {
            this.preserve_comments = value;
            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("set_main", |_, this, main: String| {
            this.main = main;
            Ok(())
        });

        methods.add_method_mut("set_encoding", |_, this, encoding: String| {
            this.encoding = encoding;
            Ok(())
        });

        methods.add_method_mut("set_output", |_, this, output: String| {
            this.output = output;
            Ok(())
        });

        methods.add_method_mut("add_module", |_, this, (module, prefix): (String, Option<String>)| {
            this.push_module(module, prefix);
            Ok(())
        });

        methods.add_method_mut("remove_module", |_, this, module: String| {
            this.remove_module(module);
            Ok(())
        });

        methods.add_method_mut("set_minify", |_, this, minify: bool| {
            this.minify = minify;
            Ok(())
        });

        methods.add_method_mut("set_preserve_comments", |_, this, preserve_comments: bool| {
            this.preserve_comments = preserve_comments;
            Ok(())
        });
    }
}