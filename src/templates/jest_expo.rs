use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "jest_config_expo.stpl")]
pub struct JestExpoConfig;
