use std::process::{Command as ProcessCommand, Output};
use std::path::Path;
use crate::command::Command;
use crate::error::CommandArgusError;

#[derive(Debug)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

impl ExecutionResult {
    fn from_output(output: Output) -> Self {
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            success: output.status.success(),
        }
    }
}

pub struct CommandExecutor;

impl CommandExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: &Command) -> Result<ExecutionResult, CommandArgusError> {
        let mut process = ProcessCommand::new(&command.command);
        
        // Add arguments
        for arg in &command.args {
            process.arg(arg);
        }
        
        // Set working directory if specified
        if let Some(ref working_dir) = command.working_directory {
            let path = Path::new(working_dir);
            if !path.exists() {
                return Err(CommandArgusError::InvalidPath(working_dir.clone()));
            }
            process.current_dir(path);
        }
        
        // On macOS, ensure common paths are included in PATH
        #[cfg(target_os = "macos")]
        {
            use std::env;
            
            let mut path_env = env::var("PATH").unwrap_or_default();
            let additional_paths = vec![
                "/opt/homebrew/bin",
                "/usr/local/bin",
                "/usr/bin",
                "/bin",
                "/usr/sbin",
                "/sbin",
            ];
            
            for additional_path in additional_paths {
                if !path_env.contains(additional_path) {
                    if !path_env.is_empty() {
                        path_env.push(':');
                    }
                    path_env.push_str(additional_path);
                }
            }
            
            process.env("PATH", path_env);
        }
        
        // Set environment variables
        for env_var in &command.environment_variables {
            process.env(&env_var.key, &env_var.value);
        }
        
        // Execute the command
        match process.output() {
            Ok(output) => Ok(ExecutionResult::from_output(output)),
            Err(e) => Err(CommandArgusError::ExecutionFailed(e.to_string())),
        }
    }
    
    pub fn execute_with_shell(&self, command: &Command) -> Result<ExecutionResult, CommandArgusError> {
        let shell_command = if cfg!(target_os = "windows") {
            "cmd"
        } else {
            "sh"
        };
        
        let shell_arg = if cfg!(target_os = "windows") {
            "/C"
        } else {
            "-c"
        };
        
        let mut process = ProcessCommand::new(shell_command);
        process.arg(shell_arg);
        process.arg(&command.full_command());
        
        // Set working directory if specified
        if let Some(ref working_dir) = command.working_directory {
            let path = Path::new(working_dir);
            if !path.exists() {
                return Err(CommandArgusError::InvalidPath(working_dir.clone()));
            }
            process.current_dir(path);
        }
        
        // On macOS, ensure common paths are included in PATH
        #[cfg(target_os = "macos")]
        {
            use std::env;
            
            let mut path_env = env::var("PATH").unwrap_or_default();
            let additional_paths = vec![
                "/opt/homebrew/bin",
                "/usr/local/bin",
                "/usr/bin",
                "/bin",
                "/usr/sbin",
                "/sbin",
            ];
            
            for additional_path in additional_paths {
                if !path_env.contains(additional_path) {
                    if !path_env.is_empty() {
                        path_env.push(':');
                    }
                    path_env.push_str(additional_path);
                }
            }
            
            process.env("PATH", path_env);
        }
        
        // Set environment variables
        for env_var in &command.environment_variables {
            process.env(&env_var.key, &env_var.value);
        }
        
        // Execute the command
        match process.output() {
            Ok(output) => Ok(ExecutionResult::from_output(output)),
            Err(e) => Err(CommandArgusError::ExecutionFailed(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_command_execution() {
        let executor = CommandExecutor::new();
        let cmd = Command::new("Echo Test".to_string(), "echo".to_string())
            .with_args(vec!["Hello, World!".to_string()]);
        
        let result = executor.execute(&cmd).unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("Hello, World!"));
    }
    
    #[test]
    fn test_command_with_invalid_working_dir() {
        let executor = CommandExecutor::new();
        let cmd = Command::new("Test".to_string(), "echo".to_string())
            .with_working_directory("/nonexistent/directory".to_string());
        
        let result = executor.execute(&cmd);
        assert!(result.is_err());
    }
}