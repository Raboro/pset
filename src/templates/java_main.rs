use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "java_main.stpl")]
pub struct JavaMain<'a> {
    pub name: &'a str,
    pub class_name: &'a str,
}
