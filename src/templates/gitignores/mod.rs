use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "gitignores/java.stpl")]
pub struct GitIgnoreJava;

#[derive(TemplateOnce)]
#[template(path = "gitignores/cli_ts.stpl")]
pub struct GitIgnoreCliTs;
