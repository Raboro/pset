use std::path::PathBuf;

use crate::{
    fs,
    templates::{
        docker_compose_spring_boot_angular::DockerComposeSpringBootAngular,
        makefile_spring_boot_angular::MakefileSpringBootAngular, Template,
    },
};

use super::{BaseProject, Project};

pub struct FullStackSpringBootAngular {
    pub base: BaseProject,
}

impl Project for FullStackSpringBootAngular {
    fn build(&self) {
        self.base.build();

        let makefile = Template::new(
            "makefile",
            "",
            None,
            &self.base.name,
            MakefileSpringBootAngular,
        );

        fs::create_file(makefile.to_path_buf(), makefile.render().unwrap())
            .expect("Cannot create Makefile");

        let docker_compose = Template::new(
            "docker-compose",
            "yml",
            None,
            &self.base.name,
            DockerComposeSpringBootAngular {
                name: &self.base.name,
            },
        );

        fs::create_file(
            docker_compose.to_path_buf(),
            docker_compose.render().unwrap(),
        )
        .expect("Cannot create docker compose");

        fs::create_file(
            PathBuf::from(format!("{}/.gitignore", self.base.name)),
            String::from("db/"),
        )
        .expect("Cannot create .gitignore");
    }
}
