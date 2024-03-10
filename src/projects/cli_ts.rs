use std::path::PathBuf;

use crate::{
    fs,
    templates::{
        ci::{Ci, CiBuilder},
        eslint::CliTsEslint,
        gitignores::GitIgnoreCliTs,
        package_json::CliTsPackageJson,
        tsconfig::TsconfigCliTs,
        Template,
    },
};

use super::{BaseProject, Project};

pub struct CliTs {
    pub base: BaseProject,
}

impl Project for CliTs {
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

        let eslint = Template::new("eslintrc", "json", None, &self.base.name, CliTsEslint);

        fs::create_file(eslint.to_path_buf(), eslint.render().unwrap())
            .expect("Cannot create eslint");

        let package_json = Template::new(
            "package",
            "json",
            None,
            &self.base.name,
            CliTsPackageJson {
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
            PathBuf::from(format!("./{}/src/index.ts", self.base.name)),
            String::from("#! /usr/bin/env node\n\n'use strict';\n\nimport 'yargs' from 'yargs';"),
        )
        .expect("Cannot create index.ts");

        let gitignore = Template::new(".gitignore", "", None, &self.base.name, GitIgnoreCliTs);

        fs::create_file(gitignore.to_path_buf(), gitignore.render().unwrap())
            .expect("Cannot create .gitignore");

        let tsconfig = Template::new("tsconfig", "json", None, &self.base.name, TsconfigCliTs);

        fs::create_file(tsconfig.to_path_buf(), tsconfig.render().unwrap())
            .expect("Cannot create tsconfig");
    }
}
