use std::path::PathBuf;

use sailfish::TemplateOnce;

pub mod basics;
pub mod ci;
pub mod ci_job;
pub mod ci_step;
pub mod cli_c;
pub mod cli_python_main;
pub mod eslint;
pub mod gitignores;
pub mod java_main;
pub mod package_json;
pub mod pom;
pub mod python_main;
pub mod tsconfig;

pub struct Template<'a, T: TemplateOnce> {
    filename: &'a str,
    file_extension: &'a str,
    file_path: Option<&'a str>,
    project_path: &'a str,
    template: T,
}

impl<'a, T: TemplateOnce> Template<'a, T> {
    pub fn new(
        filename: &'a str,
        file_extension: &'a str,
        file_path: Option<&'a str>,
        project_path: &'a str,
        template: T,
    ) -> Self {
        Self {
            filename,
            file_extension,
            file_path,
            project_path,
            template,
        }
    }

    pub fn render(self) -> Option<String> {
        self.template.render_once().ok()
    }

    pub fn to_path_buf(&self) -> PathBuf {
        self.to_path_buf_as(self.filename.to_string())
    }

    pub fn to_path_buf_with_filename(&self, filename: String) -> PathBuf {
        self.to_path_buf_as(filename)
    }

    fn to_path_buf_as(&self, filename: String) -> PathBuf {
        PathBuf::from(format!(
            "./{}/{}{}{}.{}",
            self.project_path,
            self.file_path.unwrap_or(""),
            if self.file_path.is_some() { "/" } else { "" },
            filename,
            self.file_extension
        ))
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

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{basics::License, Template};

    #[test]
    fn to_path_buf_without_file_path() -> Result<(), String> {
        let template: Template<'_, License<'_>> = Template::new(
            "test",
            "md",
            None,
            "test_dir",
            License {
                author: "Test",
                year: 2000,
            },
        );
        assert!(template
            .to_path_buf()
            .eq(&PathBuf::from("./test_dir/test.md")));
        Ok(())
    }

    #[test]
    fn to_path_buf_with_file_path() -> Result<(), String> {
        let template: Template<'_, License<'_>> = Template::new(
            "test",
            "md",
            Some("license"),
            "test_dir",
            License {
                author: "Test",
                year: 2000,
            },
        );
        assert!(template
            .to_path_buf()
            .eq(&PathBuf::from("./test_dir/license/test.md")));
        Ok(())
    }
}
