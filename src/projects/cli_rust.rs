use sailfish::TemplateOnce;

use crate::templates::{
    ci::{Ci, Job},
    ci_step::CiStep,
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
        let mut with: Vec<(String, String, Option<String>)> = vec![];
        with.push(("filter".to_string(), "fetch".to_string(), None));

        let mut env: Vec<(String, String)> = vec![];
        env.push(("gh_token".to_string(), "token".to_string()));
        let ci_step = CiStep {
            name: "Test".to_string(),
            _if: Some("Hello there".to_string()),
            run: Some(("Hello".to_string(), false)),
            uses: Some("action".to_string()),
            with: Some(with),
            env: Some(env),
        };

        let mut jobs: Vec<Job> = vec![];
        jobs.push(Job {
            name: "build".to_string(),
            steps: vec![ci_step],
        });
        let ci = Ci {
            workflow_name: "Build".to_string(),
            jobs,
        };

        println!("{}", ci.render_once().unwrap());
    }
}
