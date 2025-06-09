use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Command {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub description: Option<String>,
    pub working_directory: Option<String>,
    pub environment_variables: Vec<EnvironmentVariable>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub use_count: u32,
    pub parameters: Vec<CommandParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandParameter {
    pub name: String,
    pub placeholder: String,
    pub parameter_type: ParameterType,
    pub required: bool,
    pub default_value: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    Text,
    File,
    Directory,
    Select,
}

impl Command {
    pub fn new(name: String, command: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            command,
            args: Vec::new(),
            description: None,
            working_directory: None,
            environment_variables: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            last_used_at: None,
            use_count: 0,
            parameters: Vec::new(),
        }
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_working_directory(mut self, dir: String) -> Self {
        self.working_directory = Some(dir);
        self
    }

    pub fn add_environment_variable(&mut self, key: String, value: String) {
        self.environment_variables.push(EnvironmentVariable { key, value });
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn mark_as_used(&mut self) {
        self.last_used_at = Some(Utc::now());
        self.use_count += 1;
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn full_command(&self) -> String {
        let mut parts = vec![self.command.clone()];
        parts.extend(self.args.clone());
        parts.join(" ")
    }

    pub fn add_parameter(&mut self, parameter: CommandParameter) {
        self.parameters.push(parameter);
    }

    pub fn remove_parameter(&mut self, name: &str) {
        self.parameters.retain(|p| p.name != name);
    }

    pub fn get_parameter(&self, name: &str) -> Option<&CommandParameter> {
        self.parameters.iter().find(|p| p.name == name)
    }

    pub fn detect_placeholders(&self) -> Vec<String> {
        let mut placeholders = Vec::new();
        let full_command = self.full_command();
        
        // Match {variable} or ${variable} patterns
        let re = regex::Regex::new(r"\$?\{([^}]+)\}").unwrap();
        for cap in re.captures_iter(&full_command) {
            if let Some(name) = cap.get(1) {
                let placeholder = name.as_str().to_string();
                if !placeholders.contains(&placeholder) {
                    placeholders.push(placeholder);
                }
            }
        }
        
        placeholders
    }

    pub fn replace_placeholders(&self, values: &std::collections::HashMap<String, String>) -> (String, Vec<String>) {
        let mut command = self.command.clone();
        let mut args = self.args.clone();
        
        // Replace in command
        for (name, value) in values {
            command = command.replace(&format!("{{{}}}", name), value);
            command = command.replace(&format!("${{{}}}", name), value);
        }
        
        // Replace in args
        for arg in &mut args {
            for (name, value) in values {
                *arg = arg.replace(&format!("{{{}}}", name), value);
                *arg = arg.replace(&format!("${{{}}}", name), value);
            }
        }
        
        (command, args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_command() {
        let cmd = Command::new("List Files".to_string(), "ls".to_string());
        assert_eq!(cmd.name, "List Files");
        assert_eq!(cmd.command, "ls");
        assert_eq!(cmd.use_count, 0);
        assert!(cmd.last_used_at.is_none());
    }

    #[test]
    fn test_command_with_args() {
        let cmd = Command::new("List All".to_string(), "ls".to_string())
            .with_args(vec!["-la".to_string()]);
        assert_eq!(cmd.args, vec!["-la"]);
        assert_eq!(cmd.full_command(), "ls -la");
    }

    #[test]
    fn test_mark_as_used() {
        let mut cmd = Command::new("Test".to_string(), "echo".to_string());
        cmd.mark_as_used();
        assert_eq!(cmd.use_count, 1);
        assert!(cmd.last_used_at.is_some());
    }

    #[test]
    fn test_tags() {
        let mut cmd = Command::new("Test".to_string(), "echo".to_string());
        cmd.add_tag("development".to_string());
        cmd.add_tag("testing".to_string());
        cmd.add_tag("development".to_string()); // Duplicate
        assert_eq!(cmd.tags.len(), 2);
        
        cmd.remove_tag("testing");
        assert_eq!(cmd.tags, vec!["development"]);
    }
}