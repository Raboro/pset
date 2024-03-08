use sailfish::TemplateOnce;

use super::ci_step::CiStep;

#[derive(TemplateOnce)]
#[template(path = "ci/ci.stpl")]
pub struct Ci {
    pub workflow_name: String,
    pub jobs: Vec<Job>,
}

#[derive(Clone, Default, Debug)]
pub struct Job {
    pub name: String,
    pub steps: Vec<CiStep>,
}

impl Job {
    fn to_ref_name(&self) -> String {
        let mut chars = self.name.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct NoWorkFlowName;
#[derive(Default, Clone, Debug)]
pub struct WorkFlowName(String);

#[derive(Default, Clone, Debug)]
pub struct NoJobs;
#[derive(Default, Clone, Debug)]
pub struct Jobs(Vec<Job>);

#[derive(Clone, Default, Debug)]
pub struct CiBuilder<W, J> {
    workflow_name: W,
    jobs: J,
}

impl CiBuilder<NoWorkFlowName, NoJobs> {
    pub fn new() -> Self {
        CiBuilder::default()
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

    pub fn init_jobs(self, job: Job) -> CiBuilder<W, Jobs> {
        CiBuilder {
            workflow_name: self.workflow_name,
            jobs: Jobs(vec![job]),
        }
    }
}

impl<W> CiBuilder<W, Jobs> {
    pub fn add_job(mut self, job: Job) -> Self {
        self.jobs.0.push(job);
        self
    }
}
