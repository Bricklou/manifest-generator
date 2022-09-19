use globset::{Glob, GlobSetBuilder};

use super::Generator;

pub struct GeneratorBuilder {
    whitelist: Vec<String>,
    blacklist: Vec<String>,
    version: String,
    depends_on: Option<String>,
}

impl Default for GeneratorBuilder {
    fn default() -> Self {
        Self {
            whitelist: Vec::new(),
            blacklist: Vec::new(),
            version: String::default(),
            depends_on: None,
        }
    }
}

impl GeneratorBuilder {
    pub fn with_whitelist(mut self, whitelist: Vec<String>) -> Self {
        self.whitelist = whitelist;
        self
    }

    pub fn with_blacklist(mut self, blacklist: Vec<String>) -> Self {
        self.blacklist = blacklist;
        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    pub fn depends_on(mut self, depends_on: Option<String>) -> Self {
        self.depends_on = depends_on;
        self
    }

    pub fn build(self) -> Generator {
        let mut whitelist_glob = GlobSetBuilder::new();
        for i in self.whitelist {
            whitelist_glob.add(Glob::new(i.as_str()).unwrap());
        }
        let whitelist_glob = whitelist_glob.build().unwrap();

        let mut blacklist_glob = GlobSetBuilder::new();
        for i in self.blacklist {
            blacklist_glob.add(Glob::new(i.as_str()).unwrap());
        }
        let blacklist_glob = blacklist_glob.build().unwrap();

        let mut gen = Generator::default();
        gen.whitelist = whitelist_glob;
        gen.blacklist = blacklist_glob;
        gen.version = self.version;
        gen.depends_on = self.depends_on;

        gen
    }
}
