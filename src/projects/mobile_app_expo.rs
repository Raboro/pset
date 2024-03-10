use crate::{
    fs,
    templates::{
        babel_expo::BabelExpo,
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
    }
}
