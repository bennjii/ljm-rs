#[derive(Clone)]
pub struct LJMLua {
    script: String,
}

impl From<String> for LJMLua {
    fn from(val: String) -> Self {
        LJMLua::new(val)
    }
}

impl LJMLua {
    pub fn new<T: ToString>(module: T) -> Self {
        LJMLua {
            script: module.to_string(),
        }
    }

    pub fn size(&self) -> usize {
        self.script.len()
    }

    pub fn script(&self) -> String {
        self.script.clone()
    }
}
