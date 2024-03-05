use crate::{
    fs,
    templates::{python_main::PythonMain, Template},
};

use super::{BaseProject, Project};

pub struct BasicPython {
    pub base: BaseProject,
}

impl Project for BasicPython {
    fn build(&self) {
        self.base.build();
        fs::create_dir(&format!("./{}/src", self.base.name)).expect("Cannot create src");

        let main = Template::new("main", "py", Some("src"), &self.base.name, PythonMain {});
        fs::create_file(main.to_path_buf(), main.render().unwrap_or_default())
            .expect("Main cannot be created");
    }
}
