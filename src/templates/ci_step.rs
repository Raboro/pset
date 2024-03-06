use sailfish::TemplateOnce;

#[derive(TemplateOnce, Clone)]
#[template(path = "ci/ci_step.stpl")]
pub struct CiStep {
    pub name: String,
    pub _if: Option<String>,
    pub run: Option<(String, bool)>,
    pub uses: Option<String>,
    pub with: Option<Vec<(String, String, Option<String>)>>,
    pub env: Option<Vec<(String, String)>>,
}
