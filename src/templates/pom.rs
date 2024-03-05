use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pom.stpl")]
pub struct Pom<'a> {
    pub name: &'a str,
}
