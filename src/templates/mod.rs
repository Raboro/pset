use sailfish::TemplateOnce;

pub mod basics;

pub struct Template<'a, T: TemplateOnce> {
    filename: &'a str,
    file_extension: &'a str,
    file_path: Option<&'a str>,
    #[allow(dead_code)]
    template: T,
}

impl<'a, T: TemplateOnce> Template<'a, T> {
    #[allow(dead_code)]
    pub fn new(
        filename: &'a str,
        file_extension: &'a str,
        file_path: Option<&'a str>,
        template: T,
    ) -> Self {
        Self {
            filename,
            file_extension,
            file_path,
            template,
        }
    }

    #[allow(dead_code)]
    pub fn render(self) -> Option<String> {
        self.template.render_once().ok()
    }
}

impl<'a, T: TemplateOnce> std::fmt::Display for Template<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}.{}",
            self.file_path.unwrap_or(""),
            self.filename,
            self.file_extension
        )
    }
}
