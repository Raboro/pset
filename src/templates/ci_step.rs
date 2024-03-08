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

#[derive(Default, Clone, Debug)]
pub struct NoName;
#[derive(Default, Clone, Debug)]
pub struct Name(String);

#[derive(Default, Clone, Debug)]
pub struct NoRun;
#[derive(Default, Clone, Debug)]
pub struct Run(String);

#[derive(Default, Clone, Debug)]
pub struct NoUses;
#[derive(Default, Clone, Debug)]
pub struct Uses(String);

#[derive(Clone, Default, Debug)]
pub struct CiStepBuilder<N, R, U> {
    name: N,
    _if: Option<String>,
    run: R,
    uses: U,
    with: Option<Vec<(String, String)>>,
    env: Option<Vec<(String, String)>>,
}

impl CiStepBuilder<NoName, NoRun, NoUses> {
    pub fn new() -> Self {
        CiStepBuilder::default()
    }
}

impl CiStepBuilder<Name, Run, NoUses> {
    pub fn build(self) -> CiStep {
        CiStep {
            name: self.name.0,
            _if: self._if,
            run: Some(self.run.0),
            uses: None,
            with: None,
            env: None,
        }
    }
}

impl CiStepBuilder<Name, NoRun, Uses> {
    pub fn build(self) -> CiStep {
        CiStep {
            name: self.name.0,
            _if: self._if,
            run: None,
            uses: Some(self.uses.0),
            with: self.with,
            env: self.env,
        }
    }
}

impl<N, R, U> CiStepBuilder<N, R, U> {
    pub fn name(self, name: impl Into<String>) -> CiStepBuilder<Name, R, U> {
        CiStepBuilder {
            name: Name(name.into()),
            _if: self._if,
            run: self.run,
            uses: self.uses,
            with: self.with,
            env: self.env,
        }
    }

    pub fn _if(mut self, _if: impl Into<String>) -> Self {
        self._if = Some(_if.into());
        self
    }
}

impl<N, R> CiStepBuilder<N, R, NoUses> {
    pub fn run(self, run: impl Into<String>) -> CiStepBuilder<N, Run, NoUses> {
        CiStepBuilder {
            name: self.name,
            _if: self._if,
            run: Run(run.into()),
            uses: self.uses,
            with: None,
            env: None,
        }
    }
}

impl<N, U> CiStepBuilder<N, NoRun, U> {
    pub fn uses(self, uses: impl Into<String>) -> CiStepBuilder<N, NoRun, Uses> {
        CiStepBuilder {
            name: self.name,
            _if: self._if,
            run: self.run,
            uses: Uses(uses.into()),
            with: None,
            env: None,
        }
    }
}

impl<N> CiStepBuilder<N, NoRun, Uses> {
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
}
