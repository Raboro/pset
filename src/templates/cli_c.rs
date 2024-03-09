use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "makefile_cli_c.stpl")]
pub struct MakefileCliC<'a> {
    pub name: &'a str,
}

#[derive(TemplateOnce)]
#[template(path = "main_cli_c.stpl")]
pub struct MainC;

#[derive(TemplateOnce)]
#[template(path = "cli_parser_cli_c.stpl")]
pub struct CliParserC;

#[derive(TemplateOnce)]
#[template(path = "cli_parser_header_cli_c.stpl")]
pub struct CliParserH;
