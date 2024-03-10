use std::path::PathBuf;

use crate::{
    fs,
    templates::{
        ci::{Ci, CiBuilder},
        eslint::CliJsEslint,
        package_json::CliJsPackageJson,
        Template,
    },
};

use super::{BaseProject, Project};

pub struct CliJs {
    pub base: BaseProject,
}

impl Project for CliJs {
    fn build(&self) {
        self.base.build();

        Ci::create_dirs(&self.base.name);

        let ci_template = Template::new(
            "ci_cd",
            "yml",
            Some(".github/workflows"),
            &self.base.name,
            CiBuilder::npm_package(),
        );

        fs::create_file(
            ci_template.to_path_buf(),
            ci_template.render().unwrap().replace("&amp;#039;", "'"),
        )
        .expect("CI/CD cannot be created");

        fs::create_file(
            PathBuf::from(format!("./{}/.gitignore", self.base.name)),
            String::from("/.idea/\n/node_modules/"),
        )
        .expect("Cannot create .gitignore");

        let eslint = Template::new("eslintrc", "json", None, &self.base.name, CliJsEslint);

        fs::create_file(eslint.to_path_buf(), eslint.render().unwrap())
            .expect("Cannot create eslint");

        let package_json = Template::new(
            "package",
            "json",
            None,
            &self.base.name,
            CliJsPackageJson {
                name: &self.base.name,
                author: &self.base.author,
            },
        );

        fs::create_file(
            package_json.to_path_buf(),
            package_json.render().unwrap().replace("&amp;#039;", ","),
        )
        .expect("Cannot create package.json");

        fs::create_dir(&format!("./{}/src", self.base.name)).expect("Cannot create src folder");

        fs::create_file(
            PathBuf::from(format!("./{}/src/index.js", self.base.name)),
            String::from(
                "#! /usr/bin/env node\n\n'use strict';\n\nconst yargs = require('yargs');",
            ),
        )
        .expect("Cannot create index.js");
    }
}
