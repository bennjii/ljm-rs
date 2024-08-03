#[derive(Clone)]
pub struct LuaModule {
    script: String,
}

impl Into<LuaModule> for String {
    fn into(self) -> LuaModule {
        LuaModule::new(self)
    }
}

impl LuaModule {
    pub fn new<T: Into<String>>(module: T) -> Self {
        LuaModule {
            script: module.to_string()
        }
    }

    pub fn size(&self) -> usize {
        self.script.len()
    }

    pub fn script(&self) -> String {
        self.script.clone()
    }
}