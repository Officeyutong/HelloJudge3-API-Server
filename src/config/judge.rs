use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JudgeConfig {
    pub max_code_length: usize,
    pub compile_time_limit: i64,
    pub display_data_length_limit: usize,
    pub compile_result_length_limit: usize,
    pub spj_execute_time_limit: usize,
    pub ide_run_time_limit: usize,
    pub ide_run_memory_limit: usize,
    pub ide_run_result_length_limit: usize,
    pub ide_run_compile_parameter_length_limit: usize,
    pub auto_sync_files: bool,
    pub output_file_size_limit: usize,
}
impl Default for JudgeConfig {
    fn default() -> Self {
        Self {
            max_code_length: 100 * 1024,
            compile_time_limit: 10 * 1000,
            display_data_length_limit: 1000,
            compile_result_length_limit: 500,
            spj_execute_time_limit: 3000,
            ide_run_time_limit: 1000 * 3,
            ide_run_memory_limit: 512 * 1024 * 1024,
            ide_run_result_length_limit: 2000,
            ide_run_compile_parameter_length_limit: 30,
            auto_sync_files: true,
            output_file_size_limit: 50 * 1024 * 1024,
        }
    }
}
