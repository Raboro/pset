use super::ci_step::CiStep;

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

#[derive(Default, Clone, Debug)]
pub struct NoName;
#[derive(Default, Clone, Debug)]
pub struct Name(String);
#[derive(Default, Clone, Debug)]
pub struct NoSteps;
#[derive(Default, Clone, Debug)]
pub struct Steps(Vec<CiStep>);

#[derive(Clone, Default, Debug)]
pub struct JobBuilder<N, S> {
    name: N,
    steps: S,
}

impl JobBuilder<NoName, NoSteps> {
    pub fn new() -> Self {
        JobBuilder::default()
    }
}

impl JobBuilder<Name, Steps> {
    pub fn build(self) -> Job {
        Job {
            name: self.name.0,
            steps: self.steps.0,
        }
    }
}

impl<N, S> JobBuilder<N, S> {
    pub fn name(self, name: impl Into<String>) -> JobBuilder<Name, S> {
        JobBuilder {
            name: Name(name.into()),
            steps: self.steps,
        }
    }

    pub fn init_step(self, ci_step: CiStep) -> JobBuilder<N, Steps> {
        JobBuilder {
            name: self.name,
            steps: Steps(vec![ci_step]),
        }
    }
}

impl<N> JobBuilder<N, Steps> {
    pub fn add_step(mut self, ci_step: CiStep) -> Self {
        self.steps.0.push(ci_step);
        self
    }
}
