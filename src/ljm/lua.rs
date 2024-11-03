#[derive(Clone)]
pub struct LJMLua {
    script: String,
}

impl Into<LJMLua> for String {
    fn into(self) -> LJMLua {
        LJMLua::new(self)
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
