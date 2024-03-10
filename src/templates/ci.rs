use sailfish::TemplateOnce;

use crate::fs;

use super::{
    ci_job::{CiJob, CiJobBuilder},
    ci_step::CiStepBuilder,
};

#[derive(TemplateOnce)]
#[template(path = "ci/ci.stpl")]
pub struct Ci {
    pub workflow_name: String,
    pub jobs: Vec<CiJob>,
}

impl Ci {
    pub fn create_dirs(base: &str) {
        fs::create_nested_dirs(".github/workflows", base);
    }
}

#[derive(Default, Clone, Debug)]
pub struct NoWorkFlowName;
#[derive(Default, Clone, Debug)]
pub struct WorkFlowName(String);

#[derive(Default, Clone, Debug)]
pub struct NoJobs;
#[derive(Default, Clone, Debug)]
pub struct Jobs(Vec<CiJob>);

#[derive(Clone, Default, Debug)]
pub struct CiBuilder<W, J> {
    workflow_name: W,
    jobs: J,
}

impl CiBuilder<NoWorkFlowName, NoJobs> {
    pub fn new() -> Self {
        CiBuilder::default()
    }

    pub fn npm_package() -> Ci {
        CiBuilder::new()
        .workflow_name("CI/CD")
        .init_jobs(
            CiJobBuilder::new()
                .name("linter")
                .init_steps(CiStepBuilder::checkout())
                .add_step(
                    CiStepBuilder::new()
                        .name("Setup Node")
                        .uses("actions/setup-node@v3")
                        .with(vec![("node-version", "18")])
                        .build(),
                )
                .add_step(
                    CiStepBuilder::new()
                        .name("Install dependencies")
                        .run("npm ci")
                        .build(),
                )
                .add_step(
                    CiStepBuilder::new()
                        .name("Fix eslint issues")
                        .run("npm run lint:fix")
                        .build(),
                )
                .add_step(
                    CiStepBuilder::new()
                        .name("Check all eslint issues are fixed")
                        .run("npm run lint")
                        .build(),
                )
                .add_step(CiStepBuilder::new().name("Commit changes").run("|\n          git config user.name github-actions[bot]\n          git config user.email github-actions[bot]@users.noreply.github.com\n          git commit -am 'fixed eslint issues' || true").build())
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
                .build(),
        )
        .add_job(
            CiJobBuilder::new()
            .name("publish")
            .init_steps(CiStepBuilder::checkout())
            .add_step(
                CiStepBuilder::new()
                    .name("Setup Node")
                    .uses("actions/setup-node@v3")
                    .with(vec![("node-version", "18")])
                    .build(),
            )
            .add_step(
                CiStepBuilder::new()
                    .name("Publish to NPM")
                    .run("npm publish --access public")
                    .env(vec![("NODE_AUTH_TOKEN", "${{ secrets.NPM_AUTH_TOKEN }}")])
                    .build()
            )
            .build()
        )
        .build()
    }
}

impl CiBuilder<WorkFlowName, Jobs> {
    pub fn build(self) -> Ci {
        Ci {
            workflow_name: self.workflow_name.0,
            jobs: self.jobs.0,
        }
    }
}

impl<W, J> CiBuilder<W, J> {
    pub fn workflow_name(self, workflow_name: impl Into<String>) -> CiBuilder<WorkFlowName, J> {
        CiBuilder {
            workflow_name: WorkFlowName(workflow_name.into()),
            jobs: self.jobs,
        }
    }

    pub fn init_jobs(self, job: CiJob) -> CiBuilder<W, Jobs> {
        CiBuilder {
            workflow_name: self.workflow_name,
            jobs: Jobs(vec![job]),
        }
    }
}

impl<W> CiBuilder<W, Jobs> {
    pub fn add_job(mut self, job: CiJob) -> Self {
        self.jobs.0.push(job);
        self
    }
}
