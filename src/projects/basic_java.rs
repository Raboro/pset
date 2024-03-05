use crate::{
    fs,
    templates::{gitignores::GitIgnoreJava, java_main::JavaMain, pom::Pom, Template},
};

use super::{BaseProject, Project};

pub struct BasicJava {
    pub base: BaseProject,
}

impl BasicJava {
    fn to_class_name(&self, name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl Project for BasicJava {
    fn build(&self) {
        self.base.build();

        let pom = Template::new(
            "pom",
            "xml",
            None,
            &self.base.name,
            Pom {
                name: &self.base.name,
            },
        );

        fs::create_file(pom.to_path_buf(), pom.render().unwrap_or_default())
            .expect("Pom cannot be generated");

        let gitignore = Template::new(".gitignore", "", None, &self.base.name, GitIgnoreJava {});

        fs::create_file(
            gitignore.to_path_buf(),
            gitignore.render().unwrap_or_default(),
        )
        .expect("Gitignore cannot be generated");

        fs::create_dir(&format!("./{}/src", self.base.name)).expect("Cannot create src");
        fs::create_dir(&format!("./{}/src/main", self.base.name)).expect("Cannot create main");
        fs::create_dir(&format!("./{}/src/main/java", self.base.name)).expect("Cannot create java");
        fs::create_dir(&format!("./{}/src/main/java/io", self.base.name))
            .expect("Cannot create io");
        fs::create_dir(&format!("./{}/src/main/java/io/github", self.base.name))
            .expect("Cannot create github");
        fs::create_dir(&format!(
            "./{}/src/main/java/io/github/raboro",
            self.base.name
        ))
        .expect("Cannot create github");
        fs::create_dir(&format!(
            "./{}/src/main/java/io/github/raboro/{0}",
            self.base.name
        ))
        .expect(&format!("Cannot create {}", self.base.name));

        let file_path = &format!("src/main/java/io/github/raboro/{}", self.base.name);
        let class_name = &self.to_class_name(&self.base.name);
        let main = Template::new(
            &self.base.name,
            "java",
            Some(&file_path),
            &self.base.name,
            JavaMain {
                name: &self.base.name,
                class_name: &class_name,
            },
        );

        fs::create_file(
            main.to_path_buf_with_filename(class_name.to_string()),
            main.render().unwrap_or_default(),
        )
        .expect("Pom cannot be generated");
    }
}
