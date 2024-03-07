use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone)]
#[template(path = "ci/ci_step.stpl")]
pub struct CiStep {
    pub name: String,
    pub _if: Option<String>,
    pub run: Option<String>,
    pub uses: Option<String>,
    pub with: Option<Vec<(String, String)>>,
    pub env: Option<Vec<(String, String)>>,
}

#[derive(Clone, Default, Debug)]
pub struct CiStepBuilder {
    name: Option<String>,
    _if: Option<String>,
    run: Option<String>,
    uses: Option<String>,
    with: Option<Vec<(String, String)>>,
    env: Option<Vec<(String, String)>>,
}

impl CiStepBuilder {
    pub fn new() -> Self {
        CiStepBuilder::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn _if(mut self, _if: impl Into<String>) -> Self {
        self._if = Some(_if.into());
        self
    }

    pub fn run(mut self, run: impl Into<String>) -> Self {
        self.run = Some(run.into());
        self
    }

    pub fn uses(mut self, uses: impl Into<String>) -> Self {
        self.uses = Some(uses.into());
        self
    }

    pub fn with(
        mut self,
        with: Vec<(impl Into<String> + Clone, impl Into<String> + Clone)>,
    ) -> Self {
        self.with = Some(self.map_vec_values_to_string(with));
        self
    }

    fn map_vec_values_to_string(
        &self,
        values: Vec<(impl Into<String> + Clone, impl Into<String> + Clone)>,
    ) -> Vec<(String, String)> {
        values
            .iter()
            .map(|element| (element.0.clone().into(), element.1.clone().into()))
            .collect()
    }

    pub fn env(mut self, env: Vec<(impl Into<String> + Clone, impl Into<String> + Clone)>) -> Self {
        self.env = Some(self.map_vec_values_to_string(env));
        self
    }

    pub fn build(self) -> Option<CiStep> {
        if self.name.is_none() {
            return None;
        }

        Some(CiStep {
            name: self.name.unwrap(),
            _if: self._if,
            run: self.run,
            uses: self.uses,
            with: self.with,
            env: self.env,
        })
    }
}
