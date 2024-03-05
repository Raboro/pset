use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "basics/license.stpl")]
pub struct License<'a> {
    pub author: &'a str,
    pub year: u16,
}

#[derive(TemplateOnce)]
#[template(path = "basics/readme.stpl")]
pub struct ReadMe<'a> {
    pub project_name: &'a str,
    pub author: &'a str,
}
