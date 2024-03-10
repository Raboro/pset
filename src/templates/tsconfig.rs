use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "tsconfigs/cli_ts_tsconfig.stpl")]
pub struct TsconfigCliTs;

#[derive(TemplateOnce)]
#[template(path = "tsconfigs/expo_tsconfig.stpl")]
pub struct TsconfigExpo;
