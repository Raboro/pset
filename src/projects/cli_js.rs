use std::path::PathBuf;

use crate::{
    fs,
    templates::{
        ci::{Ci, CiBuilder},
        ci_job::CiJobBuilder,
        ci_step::CiStepBuilder,
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

        let ci = CiBuilder::new()
            .workflow_name("CI/CD")
            .init_jobs(
                CiJobBuilder::new()
                    .name("linter")
                    .init_steps(
                        CiStepBuilder::new()
                            .name("Checkout")
                            .uses("actions/checkout@v3")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Setup Node")
                            .uses("actions/setup-node@v3")
                            .with(vec![("node-version", "18")])
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Install dependencies")
                            .run("npm ci")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Fix eslint issues")
                            .run("npm run lint:fix")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Check all eslint issues are fixed")
                            .run("npm run lint")
                            .build(),
                    )
                    .add_step(CiStepBuilder::new().name("Commit changes").run("|\n          git config user.name github-actions[bot]\n          git config user.email github-actions[bot]@users.noreply.github.com\n          git commit -am 'fixed eslint issues' || true").build())
                    .add_step(
                        CiStepBuilder::new()
                            .name("Push changes")
                            .uses("ad-m/github-push-action@master")
                            .with(vec![
                                ("github_token", "${{ secrets.GITHUB_TOKEN }}"),
                                ("branch", "${{ github.ref }}"),
                            ])
                            .build(),
                    )
                    .build(),
            )
            .add_job(
                CiJobBuilder::new()
                .name("publish")
                .init_steps( 
                    CiStepBuilder::new()
                        .name("Checkout")
                        .uses("actions/checkout@v3")
                        .build()
                )
                .add_step(
                    CiStepBuilder::new()
                        .name("Setup Node")
                        .uses("actions/setup-node@v3")
                        .with(vec![("node-version", "18")])
                        .build(),
                )
                .add_step(
                    CiStepBuilder::new()
                        .name("Publish to NPM")
                        .run("npm publish --access public")
                        .env(vec![("NODE_AUTH_TOKEN", "${{ secrets.NPM_AUTH_TOKEN }}")])
                        .build()
                )
                .build()
            )
            .build();

        Ci::create_dirs(&self.base.name);

        let ci_template = Template::new(
            "ci_cd",
            "yml",
            Some(".github/workflows"),
            &self.base.name,
            ci,
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
