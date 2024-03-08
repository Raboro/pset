use super::ci_step::{CiStep};

#[derive(Clone, Default, Debug)]
pub struct Job {
    pub name: String,
    pub steps: Vec<CiStep>,
}

impl Job {
    pub fn to_ref_name(&self) -> String {
        let mut chars = self.name.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct JobBuilder {
    name: Option<String>,
    steps: Option<Vec<CiStep>>,
}

impl JobBuilder {
    pub fn new() -> Self {
        JobBuilder::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn add_step(mut self, ci_step: CiStep) -> Self {
        if let Some(steps) = self.steps.as_mut() {
            steps.push(ci_step);
        } else {
            self.steps = Some(vec![ci_step]);
        }
        self
    }

    pub fn build(self) -> Job {
        Job {
            name: self.name.unwrap(),
            steps: self.steps.unwrap(),
        }
    }
}
