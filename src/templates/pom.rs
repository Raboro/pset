use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "pom.stpl")]
pub struct Pom<'a> {
    name: &'a str,
}
