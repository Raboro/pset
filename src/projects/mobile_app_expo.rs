use crate::{
    fs,
    templates::{
        babel_expo::BabelExpo,
        ci::{Ci, CiBuilder},
        ci_job::CiJobBuilder,
        ci_step::CiStepBuilder,
        eslint::{ExpoEslint, ExpoEslintIgnore},
        gitignores::GitIgnoreExpo,
        jest_expo::JestExpoConfig,
        package_json::ExpoPackageJson,
        prettier::Prettier,
        tsconfig::TsconfigExpo,
        Template,
    },
};

use super::{BaseProject, Project};

pub struct MobileAppExpo {
    pub base: BaseProject,
}

impl Project for MobileAppExpo {
    fn build(&self) {
        self.base.build();

        let babel = Template::new("babel.config", "js", None, &self.base.name, BabelExpo);

        fs::create_file(babel.to_path_buf(), babel.render().unwrap()).expect("Cannot create Babel");

        let jest = Template::new("jest.config", ".js", None, &self.base.name, JestExpoConfig);

        fs::create_file(jest.to_path_buf(), jest.render().unwrap()).expect("Cannot create jest");

        let gitignore = Template::new(".gitignore", "", None, &self.base.name, GitIgnoreExpo);

        fs::create_file(gitignore.to_path_buf(), gitignore.render().unwrap())
            .expect("Cannot create .gitignore");

        let prettier = Template::new("prettierrc", "json", None, &self.base.name, Prettier);

        fs::create_file(prettier.to_path_buf(), prettier.render().unwrap())
            .expect("Cannot create prettier");

        let tsconfig = Template::new("tsconfig", "json", None, &self.base.name, TsconfigExpo);

        fs::create_file(tsconfig.to_path_buf(), tsconfig.render().unwrap())
            .expect("Cannot create tsconfig");

        let package_json = Template::new(
            "package",
            "json",
            None,
            &self.base.name,
            ExpoPackageJson {
                name: &self.base.name,
            },
        );

        fs::create_file(package_json.to_path_buf(), package_json.render().unwrap())
            .expect("Cannot create package.json");

        let eslint = Template::new(".eslintrc", "", None, &self.base.name, ExpoEslint);

        fs::create_file(eslint.to_path_buf(), eslint.render().unwrap())
            .expect("Cannot create eslint");

        let eslint_ignore =
            Template::new(".eslintignore", "", None, &self.base.name, ExpoEslintIgnore);

        fs::create_file(eslint_ignore.to_path_buf(), eslint_ignore.render().unwrap())
            .expect("Cannot create .eslintignore");

        if !self.base.generate_ci {
            return;
        }

        let ci = CiBuilder::new()
            .workflow_name("CI")
            .init_jobs(
                CiJobBuilder::new()
                    .name("organizeImports_lint_format_test")
                    .init_steps(CiStepBuilder::checkout())
                    .add_step(
                        CiStepBuilder::new()
                            .name("Setup bun")
                            .uses("oven-sh/setup-bun@v1")
                            .with(vec![("bun-version", "latest")])
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Install dependencies")
                            .run("bun install")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Organize imports")
                            .run("bun run organizeImports")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Fix eslint issues")
                            .run("bun run lint:fix")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Check if all eslint issues are fixed")
                            .run("bun run lint")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Format with prettier")
                            .run("bun run format")
                            .build(),
                    )
                    .add_step(CiStepBuilder::new().name("Commit changes").run("|\n          git config user.name github-actions[bot]\n          git config user.email github-actions[bot]@users.noreply.github.com\n          git commit -am 'fixed eslint issues, formatted with prettier and organized imports' || true").build())
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
                    .add_step(CiStepBuilder::new().name("Setup node to run tests").uses("actions/setup-node@v3").with(vec![("node-version", "18")]).build())
                    .add_step(CiStepBuilder::new().name("Tests with Jest").run("npm test").build())
                    .build(),
            )
            .add_job(
                CiJobBuilder::new()
                    .name("Sonar")
                    .init_steps(CiStepBuilder::checkout())
                    .add_step(
                        CiStepBuilder::new()
                            .name("Fetch changed ts/tsx/js/ci.yml files")
                            .id("changedFiles")
                            .uses("Ana06/get-changed-files@v2.2.0")
                            .with(vec![("filter", "|\n            *.ts\n            *.tsx\n            *.js\n            ci.yml")])
                            .build()
                    )
                    .add_step(
                        CiStepBuilder::new()
                        .name("Check Sonar trigger")
                        .run("|\n          if [ -n '${{ steps.changedFiles.outputs.all }}' ]; then\n            echo 'TRIGGER_SONAR=true' >> '$GITHUB_ENV'\n          else\n            echo 'TRIGGER_SONAR=false' >> 'GITHUB_ENV'\n          fi")
                        .build()
                    )
                    .add_step(
                        CiStepBuilder::new()
                        .name("Setup bun")
                        .uses("oven-sh/setup-bun@v1")
                        ._if("env.TRIGGER_SONAR == 'true'")
                        .with(vec![("bun-version", "latest")])
                        .build()
                    )
                    .add_step(CiStepBuilder::new()
                        .name("Setup node to run tests")
                        ._if("env.TRIGGER_SONAR == 'true'")
                        .uses("actions/setup-node@v3")
                        .with(vec![("node-version", "18")])
                        .build()
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Install dependencies")
                            ._if("env.TRIGGER_SONAR == 'true'")
                            .run("bun install")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                        .name("Generate Test Coverage Report")
                        ._if("env.TRIGGER_SONAR == 'true'")
                        .run("npm run test:coverage")
                        .build()
                    )
                    .add_step(
                        CiStepBuilder::new()
                        .name("Sonar Scan")
                        .uses("SonarSource/sonarcloud-github-action@master")
                        ._if("env.TRIGGER_SONAR == 'true'")
                        .env(vec![("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}"), ("SONAR_TOKEN", "${{ secrets.SONAR_TOKEN }}")])
                        .with(vec![("args", format!("switch>\n            -Dsonar.organization=raboro\n            -Dsonar.projectKey=Raboro_{}\n            -Dsonar.javascript.lcov.reportPaths=./coverage/lcov.info", self.base.name))])
                        .build()
                    )
                    .build()
            )
            .build();

        Ci::create_dirs(&self.base.name);

        let ci_template =
            Template::new("ci", "yml", Some(".github/workflows"), &self.base.name, ci);

        fs::create_file(
            ci_template.to_path_buf(),
            ci_template
                .render()
                .unwrap()
                .replace("&amp;#039;", "'") // sailfish cannot render ' correctly
                .replace("&amp;gt;", "=") // sailfish cannot render = correctly
                .replace("switch=", ">"), // sailfish cannot render > correctly
        )
        .expect("Ci cannot be generated");
    }
}
