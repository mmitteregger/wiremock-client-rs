use serde::{Deserialize, Serialize};

use crate::stubbing::StubMapping;

pub fn stub_import() -> StubImportBuilder {
    StubImportBuilder::new()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StubImport {
    mappings: Vec<StubMapping>,
    import_options: Options,
}

impl StubImport {
    pub fn mappings(&self) -> &[StubMapping] {
        &self.mappings
    }

    pub fn import_options(&self) -> Options {
        self.import_options
    }
}

impl From<StubImportBuilder> for StubImport {
    fn from(builder: StubImportBuilder) -> StubImport {
        builder.build()
    }
}

impl Into<Vec<StubMapping>> for StubImport {
    fn into(self) -> Vec<StubMapping> {
        self.mappings
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Options {
    duplicate_policy: DuplicatePolicy,
    delete_all_not_in_import: bool,
}

impl Options {
    pub const DEFAULTS: Options = Options {
        duplicate_policy: DuplicatePolicy::OVERWRITE,
        delete_all_not_in_import: false,
    };
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum DuplicatePolicy {
    OVERWRITE,
    IGNORE,
}

pub struct StubImportBuilder {
    mappings: Vec<StubMapping>,
    import_options: Options,
}

impl StubImportBuilder {
    fn new() -> StubImportBuilder {
        StubImportBuilder {
            mappings: Vec::new(),
            import_options: Options::DEFAULTS,
        }
    }

    pub fn stub<S>(mut self, stub_mapping: S) -> StubImportBuilder where S: Into<StubMapping> {
        self.mappings.push(stub_mapping.into());
        self
    }

    pub fn ignore_existing(mut self) -> StubImportBuilder {
        self.import_options.duplicate_policy = DuplicatePolicy::IGNORE;
        self
    }

    pub fn overwrite_existing(mut self) -> StubImportBuilder {
        self.import_options.duplicate_policy = DuplicatePolicy::OVERWRITE;
        self
    }

    pub fn delete_all_existing_stubs_not_in_import(mut self) -> StubImportBuilder {
        self.import_options.delete_all_not_in_import = true;
        self
    }

    pub fn do_not_delete_existing_stubs(mut self) -> StubImportBuilder {
        self.import_options.delete_all_not_in_import = false;
        self
    }

    pub fn build(self) -> StubImport {
        StubImport {
            mappings: self.mappings,
            import_options: self.import_options,
        }
    }
}
