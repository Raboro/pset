use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "gitignores/java.stpl")]
pub struct GitIgnoreJava;