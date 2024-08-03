#[derive(Clone)]
pub struct LuaModule {
    script: String,
}

impl LuaModule {
    pub fn new(module: &str) -> Self {
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