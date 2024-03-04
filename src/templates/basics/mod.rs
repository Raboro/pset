use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "basics/license.stpl")]
pub struct License<'a> {
    pub author: &'a str,
    pub year: u16,
}
