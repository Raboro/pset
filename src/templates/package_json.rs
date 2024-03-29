use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "package_json/cli_js_package_json.stpl")]
pub struct CliJsPackageJson<'a> {
    pub name: &'a str,
    pub author: &'a str,
}

#[derive(TemplateOnce)]
#[template(path = "package_json/cli_ts_package_json.stpl")]
pub struct CliTsPackageJson<'a> {
    pub name: &'a str,
    pub author: &'a str,
}

#[derive(TemplateOnce)]
#[template(path = "package_json/expo_package_json.stpl")]
pub struct ExpoPackageJson<'a> {
    pub name: &'a str,
}
