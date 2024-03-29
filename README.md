# pset

CLI tool to generate projects.

## Usage
```bash
Usage: pset [OPTIONS] --name <NAME> --project-type <PROJECT_TYPE> --author <AUTHOR>

Options:
  -n, --name <NAME>                  Name of the project
  -p, --project-type <PROJECT_TYPE>  Type of the project [possible values: mobile-app-expo, cli-c, cli-rust, cli-python, cli-js, cli-ts, obsidian-plugin, basic-java, basic-python, full-stack-spring-boot-angular]
  -c, --ci                           Generate GitHub action for CI (ci.yml), default false
  -a, --author <AUTHOR>              author of the project
  -h, --help                         Print help
  -V, --version                      Print version
```

## Roadmap 
[x] basic_java \
[x] basic_python \
[x] cli_c \
[x] cli_js \
[x] cli_python \
[x] cli_rust \
[x] cli_ts \
[/] full_stack_spring_boot_angular \
[x] mobile_app_expo \
[x] obsidian_plugin