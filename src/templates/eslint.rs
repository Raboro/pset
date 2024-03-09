use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "eslint/cli_js_eslint.stpl")]
pub struct CliJsEslint;

#[derive(TemplateOnce)]
#[template(path = "eslint/cli_ts_eslint.stpl")]
pub struct CliTsEslint;
