use std::collections::HashMap;
use testcontainers::{Container, Docker, Image};

const CONTAINER_IDENTIFIER: &str = "eventstore/eventstore";
const DEFAULT_TAG: &str = "20.10.0-bionic";

#[derive(Debug, Default, Clone)]
pub struct ESDBArgs;

impl IntoIterator for ESDBArgs {
    type Item = String;
    type IntoIter = ::std::vec::IntoIter<String>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        vec![].into_iter()
    }
}

#[derive(Debug)]
pub struct ESDB {
    tag: String,
    arguments: ESDBArgs,
}

impl Image for ESDB {
    type Args = ESDBArgs;
    type EnvVars = HashMap<String, String>;
    type Volumes = HashMap<String, String>;
    type EntryPoint = std::convert::Infallible;

    fn descriptor(&self) -> String {
        format!("{}:{}", CONTAINER_IDENTIFIER, &self.tag)
    }

    fn wait_until_ready<D: Docker>(&self, _: &Container<'_, D, Self>) {
        // container.logs().stdout.wait_for_message("SPARTA!").unwrap();
    }

    fn args(&self) -> Self::Args {
        self.arguments.clone()
    }

    fn env_vars(&self) -> Self::EnvVars {
        HashMap::new()
    }

    fn volumes(&self) -> Self::Volumes {
        HashMap::new()
    }

    fn with_args(self, arguments: Self::Args) -> Self {
        ESDB { arguments, ..self }
    }
}

impl Default for ESDB {
    fn default() -> Self {
        ESDB {
            tag: DEFAULT_TAG.to_string(),
            ..Default::default()
        }
    }
}


fn main() {
    let _ = pretty_env_logger::try_init();
    let docker = testcontainers::clients::Cli::default();
    let args = testcontainers::RunArgs::default().with_mapped_port((2_113, 2_113));
    let _containter = docker.run_with_args(ESDB::default(), args);

    std::thread::sleep(std::time::Duration::from_secs(500_000));
    format!("{:?}", _containter.get_host_port(1_1113));
}
