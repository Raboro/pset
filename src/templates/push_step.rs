use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "ci/push_step.stpl")]
pub struct PushStep<'a> {
    pub commit_message: &'a str,
}
