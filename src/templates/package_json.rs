use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "package_json/cli_js_package_json.stpl")]
pub struct CliJsPackageJson<'a> {
    pub name: &'a str,
}
