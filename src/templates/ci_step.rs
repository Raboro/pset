use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone, Default, Debug)]
#[template(path = "ci/ci_step.stpl")]
pub struct CiStep {
    pub name: String,
    pub _if: Option<String>,
    pub run: Option<String>,
    pub uses: Option<String>,
    pub with: Option<Vec<(String, String)>>,
    pub env: Option<Vec<(String, String)>>,
    pub id: Option<String>,
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
    id: Option<String>,
}

impl CiStepBuilder<NoName, NoRun, NoUses> {
    pub fn new() -> Self {
        CiStepBuilder::default()
    }

    pub fn checkout() -> CiStep {
        CiStep {
            name: String::from("Checkout"),
            _if: None,
            run: None,
            uses: Some(String::from("actions/checkout@v3")),
            with: None,
            env: None,
            id: None,
        }
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
            env: self.env,
            id: self.id,
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
            id: self.id,
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
            id: self.id,
        }
    }

    pub fn _if(mut self, _if: impl Into<String>) -> Self {
        self._if = Some(_if.into());
        self
    }

    pub fn env(mut self, env: Vec<(impl Into<String> + Clone, impl Into<String> + Clone)>) -> Self {
        self.env = Some(self.map_vec_values_to_string(env));
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
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
}

impl<N, R> CiStepBuilder<N, R, NoUses> {
    pub fn run(self, run: impl Into<String>) -> CiStepBuilder<N, Run, NoUses> {
        CiStepBuilder {
            name: self.name,
            _if: self._if,
            run: Run(run.into()),
            uses: self.uses,
            with: None,
            env: self.env,
            id: self.id,
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
            env: self.env,
            id: self.id,
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
}
