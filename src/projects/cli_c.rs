use std::path::PathBuf;

use crate::{
    fs,
    templates::{
        ci::{Ci, CiBuilder},
        ci_job::CiJobBuilder,
        ci_step::CiStepBuilder,
        cli_c::*,
        Template,
    },
};

use super::{BaseProject, Project};

pub struct CliC {
    pub base: BaseProject,
}

impl Project for CliC {
    fn build(&self) {
        self.base.build();

        fs::create_file(
            PathBuf::from(format!("./{}/.gitignore", self.base.name)),
            String::from("**/*.o\n**/*.exe"),
        )
        .expect("Cannot create .gitignore");

        fs::create_dir(&format!("./{}/src", self.base.name)).expect("Cannot create folder src");

        let makefile = Template::new(
            "makefile",
            "",
            None,
            &self.base.name,
            MakefileCliC {
                name: &self.base.name,
            },
        );

        fs::create_file(makefile.to_path_buf(), makefile.render().unwrap())
            .expect("makefile cannot be created");

        let ci = CiBuilder::new()
            .workflow_name("Build")
            .init_jobs(
                CiJobBuilder::new()
                    .name("build")
                    .init_steps(
                        CiStepBuilder::new()
                            .name("Update")
                            .run("sudo apt-get update")
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Install gcc")
                            .run("sudo apt-get install -y gcc")
                            .build(),
                    )
                    .add_step(CiStepBuilder::checkout())
                    .add_step(CiStepBuilder::new().name("Build").run("make").build())
                    .build(),
            )
            .build();

        let ci_template = Template::new(
            "build",
            "yml",
            Some(".github/workflows"),
            &self.base.name,
            ci,
        );

        Ci::create_dirs(&self.base.name);

        fs::create_file(ci_template.to_path_buf(), ci_template.render().unwrap())
            .expect("Cannot create build.yml");

        let main = Template::new("main", "c", Some("src"), &self.base.name, MainC {});

        fs::create_file(main.to_path_buf(), main.render().unwrap()).expect("Cannot create main");

        let cli_parser = Template::new(
            "cli_parser",
            "c",
            Some("src"),
            &self.base.name,
            CliParserC {},
        );

        fs::create_file(cli_parser.to_path_buf(), cli_parser.render().unwrap())
            .expect("Cannot create cli_parser.h");

        let cli_parser_h = Template::new(
            "cli_parser",
            "h",
            Some("src"),
            &self.base.name,
            CliParserH {},
        );

        fs::create_file(cli_parser_h.to_path_buf(), cli_parser_h.render().unwrap())
            .expect("Cannot create cli_parser.h");
    }
}
