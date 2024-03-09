use std::process::Command;

use crate::{
    fs,
    templates::{
        ci::{self, Ci, CiBuilder},
        ci_job::CiJobBuilder,
        ci_step::CiStepBuilder,
        Template,
    },
};

use super::{BaseProject, Project};

pub struct CliRust {
    pub base: BaseProject,
}

impl Project for CliRust {
    fn build(&self) {
        Command::new("cargo")
            .arg("new")
            .arg(&self.base.name)
            .output()
            .expect("Cannot create cargo project");
        self.base.create_license();
        self.base.create_readme();

        let ci: ci::Ci = CiBuilder::new()
            .workflow_name("CI")
            .init_jobs(
                CiJobBuilder::new()
                    .name("build")
                    .init_steps(CiStepBuilder::new().name("Checkout").uses("actions/checkout@v3").build())
                    .add_step(
                        CiStepBuilder::new()
                            .name("Check")
                            .run("cargo check")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Clippy fix")
                            .run("cargo clippy --fix")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Check no more Clippy errors")
                            .run("cargo clippy -- -D warnings")
                            .build(),
                    )
                    .add_step(CiStepBuilder::new().name("Format").run("cargo fmt").build())
                    .add_step(CiStepBuilder::new().name("Commit changes").run("|\n          git config user.name github-actions[bot]\n          git config user.email github-actions[bot]@users.noreply.github.com\n          git commit -am 'formatted and clippy' || true").build())
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
                    .add_step(
                        CiStepBuilder::new()
                            .name("Build")
                            .run("cargo build")
                            .build(),
                    )
                    .add_step(CiStepBuilder::new().name("Test").run("cargo test").build())
                    .build(),
            )
            .build();

        Ci::create_dirs(&self.base.name);

        let ci_template =
            Template::new("ci", "yml", Some(".github/workflows"), &self.base.name, ci);

        fs::create_file(
            ci_template.to_path_buf(),
            ci_template.render().unwrap().replace("&amp;#039;", "'"), // sailfish cannot render ' correctly
        )
        .expect("Ci cannot be generated");
    }
}
