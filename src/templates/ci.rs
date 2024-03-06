use sailfish::TemplateOnce;

use super::ci_step::CiStep;

#[derive(TemplateOnce)]
#[template(path = "ci/ci.stpl")]
pub struct Ci {
    pub workflow_name: String,
    pub jobs: Vec<Job>,
}

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
