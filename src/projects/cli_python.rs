use crate::{
    fs,
    templates::{cli_python_main::CliPythonMain, Template},
};

use super::{BaseProject, Project};

pub struct CliPython {
    pub base: BaseProject,
}

impl Project for CliPython {
    fn build(&self) {
        self.base.build();
        fs::create_dir(&format!("./{}/src", self.base.name)).expect("Cannot create src");

        let main = Template::new("main", "py", Some("src"), &self.base.name, CliPythonMain {});
        fs::create_file(main.to_path_buf(), main.render().unwrap_or_default())
            .expect("Main cannot be created");
    }
}
