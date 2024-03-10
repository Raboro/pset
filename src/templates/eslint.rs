use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "eslint/cli_js_eslint.stpl")]
pub struct CliJsEslint;

#[derive(TemplateOnce)]
#[template(path = "eslint/cli_ts_eslint.stpl")]
pub struct CliTsEslint;

#[derive(TemplateOnce)]
#[template(path = "eslint/expo_eslint.stpl")]
pub struct ExpoEslint;

#[derive(TemplateOnce)]
#[template(path = "eslint/expo_eslintignore.stpl")]
pub struct ExpoEslintIgnore;
