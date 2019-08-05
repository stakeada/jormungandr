use super::logger::JormungandrLogger;
use crate::common::configuration::jormungandr_config::JormungandrConfig;
use crate::common::jcli_wrapper;
use crate::common::{process_assert, process_utils};
use std::path::PathBuf;
use std::process::Child;

#[derive(Debug)]
pub struct JormungandrProcess {
    pub child: Child,
    pub logger: JormungandrLogger,
    pub config: JormungandrConfig,
    description: String,
}

impl JormungandrProcess {
    pub fn from_config(child: Child, config: JormungandrConfig) -> Self {
        JormungandrProcess::new(
            child,
            String::from("Jormungandr node"),
            config.log_file_path.clone(),
            config,
        )
    }

    pub fn new(
        child: Child,
        description: String,
        log_file_path: PathBuf,
        config: JormungandrConfig,
    ) -> Self {
        JormungandrProcess {
            child: child,
            description: description,
            logger: JormungandrLogger::new(log_file_path.clone()),
            config: config,
        }
    }

    pub fn assert_no_errors_in_log(&self) {
        let error_lines = self.logger.get_lines_with_error().collect::<Vec<String>>();

        assert_eq!(
            error_lines.len(),
            0,
            "there are some errors in log ({:?}): {:?}",
            self.logger.log_file_path,
            error_lines
        );
    }
}

impl Drop for JormungandrProcess {
    fn drop(&mut self) {
        self.logger.print_error_and_invalid_logs();
        assert_shutdown_node(&self.config.get_node_address());
    }
}

/// Method sends shutdown signal to jormungandr REST API
/// WARNING: It asserts that REST API response is ok, it does not verify if its still up
fn assert_shutdown_node(host: &str) {
    let output = process_utils::run_process_and_get_output(
        jcli_wrapper::jcli_commands::get_rest_shutdown_node_command(&host),
    );
    process_assert::assert_process_exited_successfully(output);
}
