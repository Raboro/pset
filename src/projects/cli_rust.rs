use sailfish::TemplateOnce;

use crate::templates::{
    ci::CiBuilder,
    ci_step::{CiStep, CiStepBuilder},
    job::{Job, JobBuilder},
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

        let builder = CiStepBuilder::new()
            .name("Test")
            ._if("is null")
            .uses("checkout")
            .with(vec![("Test", "hello2"), ("Test2", "hello2")])
            .env(vec![("Hi", "hii")]);
        println!("{:#?}", builder);
        println!("{}", builder.build().render_once().unwrap());

        let builder2 = CiStepBuilder::new()
            .name("Test")
            ._if("is null")
            .run("npm run dev");
        println!("{:#?}", builder2);
        println!("{}", builder2.build().render_once().unwrap());

        let ci = CiBuilder::new()
            .workflow_name("Build")
            .init_jobs(jobs.get(0).unwrap().to_owned())
            .add_job(jobs.get(2).unwrap().to_owned());

        println!("\n\n{}", ci.build().render_once().unwrap());

        let job = JobBuilder::new()
            .name("Test")
            .init_step(CiStepBuilder::new().name("test").run("npm test").build())
            .add_step(
                CiStepBuilder::new()
                    .name("build")
                    .run("npm  run build")
                    .build(),
            );

        println!("{:#?}", job.build());
    }
}
