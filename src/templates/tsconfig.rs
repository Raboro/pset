use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "tsconfigs/cli_ts_tsconfig.stpl")]
pub struct TsconfigCliTs;
