use crate::{
    fs,
    templates::{
        ci::{Ci, CiBuilder},
        ci_job::CiJobBuilder,
        ci_step::CiStepBuilder,
        gitignores::GitIgnoreJava,
        java_main::JavaMain,
        pom::Pom,
        Template,
    },
};

use super::{BaseProject, Project};

pub struct BasicJava {
    pub base: BaseProject,
}

impl BasicJava {
    fn to_class_name(&self, name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl Project for BasicJava {
    fn build(&self) {
        self.base.build();

        let pom = Template::new(
            "pom",
            "xml",
            None,
            &self.base.name,
            Pom {
                name: &self.base.name,
            },
        );

        fs::create_file(pom.to_path_buf(), pom.render().unwrap_or_default())
            .expect("Pom cannot be generated");

        let gitignore = Template::new(".gitignore", "", None, &self.base.name, GitIgnoreJava {});

        fs::create_file(
            gitignore.to_path_buf(),
            gitignore.render().unwrap_or_default(),
        )
        .expect("Gitignore cannot be generated");

        fs::create_nested_dirs(
            &format!("src/main/java/io/github/raboro/{0}", self.base.name),
            &self.base.name,
        );

        let file_path = &format!("src/main/java/io/github/raboro/{}", self.base.name);
        let class_name = &self.to_class_name(&self.base.name);
        let main = Template::new(
            &self.base.name,
            "java",
            Some(file_path),
            &self.base.name,
            JavaMain {
                name: &self.base.name,
                class_name,
            },
        );

        fs::create_file(
            main.to_path_buf_with_filename(class_name.to_string()),
            main.render().unwrap_or_default(),
        )
        .expect("Pom cannot be generated");

        let ci: Ci = CiBuilder::new()
            .workflow_name("CI")
            .init_jobs(
                CiJobBuilder::new()
                    .name("build")
                    .init_steps(CiStepBuilder::checkout())
                    .add_step(
                        CiStepBuilder::new()
                            .name("Set up JDK 17")
                            .uses("actions/setup-java@v3")
                            .with(vec![
                                ("java-version", "17"),
                                ("distribution", "temurin"),
                                ("cache", "maven"),
                            ])
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Build with Maven")
                            .run("mvn -B package --file pom.xml")
                            .build(),
                    )
                    .build(),
            )
            .add_job(
                CiJobBuilder::new()
                    .name("sonar")
                    .init_steps(
                        CiStepBuilder::checkout())
                    .add_step(
                        CiStepBuilder::new()
                            .name("Set up JDK 17")
                            .uses("actions/setup-java@v3")
                            .with(vec![
                                ("java-version", "17"),
                                ("distribution", "temurin"),
                                ("cache", "maven"),
                            ])
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Cache SonarCloud packages")
                            .uses("actions/cache@v3")
                            .with(vec![
                                ("path", "~/.sonar/cache"),
                                ("key", "${{ runner.os }}-sonar"),
                                ("restore-keys", "${{ runner.os }}-sonar"),
                            ])
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Cache Maven packages")
                            .uses("actions/cache@v3")
                            .with(vec![
                                ("path", "~/.m2"),
                                ("key", "${{ runner.os }}-m2-${{ 'hashFiles(**/pom.xml)' }}"),
                                ("restore-keys", "${{ runner.os }}-m2"),
                            ])
                            .build(),
                    )
                    .add_step(
                        CiStepBuilder::new()
                            .name("Build and analyze with Sonar")
                            .run(format!("mvn -B verify org.sonarsource.scanner.maven:sonar-maven-plugin:sonar -Dsonar.projectKey=Raboro_{}", self.base.name))
                            .env(vec![("GITHUB_TOKEN", "${{ secrets.GITHUB_TOKEN }}"), ("SONAR_TOKEN", "${{ secrets.SONAR_TOKEN }}")])
                            .build(),
                    )
                    .build(),
            )
            .build();

        Ci::create_dirs(&self.base.name);

        let ci_template =
            Template::new("ci", "yml", Some(".github/workflows"), &self.base.name, ci);

        fs::create_file(
            ci_template.to_path_buf(),
            ci_template.render().unwrap().replace("&amp;#039;", "'"), // sailfish cannot render ' correctly
        )
        .expect("Ci cannot be created");
    }
}
