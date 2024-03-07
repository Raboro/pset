use sailfish::TemplateOnce;

use crate::templates::{
    ci::{Ci, Job},
    ci_step::{CiStep, CiStepBuilder},
};

use super::{BaseProject, Project};

pub struct CliRust {
    pub base: BaseProject,
}

impl Project for CliRust {
    fn build(&self) {
        /*Command::new("cargo")
            .arg("new")
            .arg(&self.base.name)
            .output()
            .expect("Cannot create cargo project");
        self.base.create_license();
        self.base.create_readme();
        */
        let with: Vec<(String, String)> = vec![("filter".to_string(), "fetch".to_string())];

        let env: Vec<(String, String)> = vec![("gh_token".to_string(), "token".to_string())];
        let ci_step = CiStep {
            name: "Test".to_string(),
            _if: Some("Hello there".to_string()),
            run: Some("Hello".to_string()),
            uses: Some("action".to_string()),
            with: Some(with),
            env: Some(env),
        };

        let with_long: Vec<(String, String)> = vec![
            ("filter".to_string(), "fetch".to_string()),
            ("filter".to_string(), "decode".to_string()),
        ];

        let env_long: Vec<(String, String)> = vec![
            ("gh_token".to_string(), "token".to_string()),
            ("sonar_token".to_string(), "sonar_token".to_string()),
        ];

        let ci_step_big = CiStep {
            name: "Test".to_string(),
            _if: Some("Hello there".to_string()),
            run: Some("Hello".to_string()),
            uses: Some("action".to_string()),
            with: Some(with_long),
            env: Some(env_long),
        };

        let jobs: Vec<Job> = vec![
            Job {
                name: "build".to_string(),
                steps: vec![ci_step.clone()],
            },
            Job {
                name: "test".to_string(),
                steps: vec![ci_step.clone(), ci_step.clone()],
            },
            Job {
                name: "verify".to_string(),
                steps: vec![ci_step_big],
            },
        ];
        let ci = Ci {
            workflow_name: "Build".to_string(),
            jobs,
        };

        println!("{}", ci.render_once().unwrap());
        let builder = CiStepBuilder::new()
            .name("Test")
            ._if("is null")
            .run("npm test")
            .uses("checkout")
            .with(vec![("Test", "hello2"), ("Test2", "hello2")])
            .env(vec![("Hi", "hii")]);
        println!("{:#?}", builder);
        println!("{}", builder.build().unwrap().render_once().unwrap());
    }
}
