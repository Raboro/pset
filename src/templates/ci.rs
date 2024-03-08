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

#[derive(Clone, Default, Debug)]
pub struct CiBuilder {
    workflow_name: Option<String>,
    jobs: Option<Vec<Job>>,
}

impl CiBuilder {
    pub fn new() -> Self {
        CiBuilder::default()
    }

    pub fn workflow_name(mut self, workflow_name: impl Into<String>) -> Self {
        self.workflow_name = Some(workflow_name.into());
        self
    }

    pub fn add_job(mut self, job: Job) -> Self {
        if let Some(jobs) = self.jobs.as_mut() {
            jobs.push(job);
        } else {
            self.jobs = Some(vec![job]);
        }
        self
    }

    pub fn build(self) -> Ci {
        Ci {
            workflow_name: self.workflow_name.unwrap(),
            jobs: self.jobs.unwrap(),
        }
    }
}
