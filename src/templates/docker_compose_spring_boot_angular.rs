use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "docker_compose_spring_boot_angular.stpl")]
pub struct DockerComposeSpringBootAngular<'a> {
    pub name: &'a str,
}
