use crate::{Command, CommandArgusError, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub struct CommandStorage {
    storage_path: PathBuf,
}

impl CommandStorage {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "command-argus", "command-argus")
            .ok_or_else(|| CommandArgusError::Storage("Failed to get project directories".to_string()))?;
        
        let storage_dir = proj_dirs.data_dir();
        fs::create_dir_all(storage_dir)?;
        
        let storage_path = storage_dir.join("commands.json");
        
        Ok(Self { storage_path })
    }

    pub fn with_path(path: PathBuf) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(Self { storage_path: path })
    }

    pub fn create(&self, command: Command) -> Result<Command> {
        let mut commands = self.load_all()?;
        
        // Check for duplicate names
        if commands.iter().any(|c| c.name == command.name) {
            return Err(CommandArgusError::DuplicateName(command.name.clone()));
        }
        
        commands.push(command.clone());
        self.save_all(&commands)?;
        
        Ok(command)
    }

    pub fn read(&self, id: Uuid) -> Result<Command> {
        let commands = self.load_all()?;
        commands.into_iter()
            .find(|c| c.id == id)
            .ok_or(CommandArgusError::NotFound(id))
    }

    pub fn read_by_name(&self, name: &str) -> Result<Command> {
        let commands = self.load_all()?;
        commands.into_iter()
            .find(|c| c.name == name)
            .ok_or_else(|| CommandArgusError::Storage(format!("Command with name '{}' not found", name)))
    }

    pub fn update(&self, id: Uuid, mut update_fn: impl FnMut(&mut Command)) -> Result<Command> {
        let mut commands = self.load_all()?;
        
        let command = commands.iter_mut()
            .find(|c| c.id == id)
            .ok_or(CommandArgusError::NotFound(id))?;
        
        update_fn(command);
        command.update();
        
        let updated_command = command.clone();
        self.save_all(&commands)?;
        
        Ok(updated_command)
    }

    pub fn delete(&self, id: Uuid) -> Result<()> {
        let mut commands = self.load_all()?;
        let initial_len = commands.len();
        
        commands.retain(|c| c.id != id);
        
        if commands.len() == initial_len {
            return Err(CommandArgusError::NotFound(id));
        }
        
        self.save_all(&commands)?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Command>> {
        self.load_all()
    }

    pub fn search_by_tags(&self, tags: &[String]) -> Result<Vec<Command>> {
        let commands = self.load_all()?;
        Ok(commands.into_iter()
            .filter(|c| tags.iter().any(|tag| c.tags.contains(tag)))
            .collect())
    }

    pub fn search_by_name(&self, query: &str) -> Result<Vec<Command>> {
        let commands = self.load_all()?;
        let query_lower = query.to_lowercase();
        Ok(commands.into_iter()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .collect())
    }

    fn load_all(&self) -> Result<Vec<Command>> {
        if !self.storage_path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&self.storage_path)?;
        let commands: Vec<Command> = serde_json::from_str(&content)?;
        Ok(commands)
    }

    fn save_all(&self, commands: &[Command]) -> Result<()> {
        let content = serde_json::to_string_pretty(commands)?;
        fs::write(&self.storage_path, content)?;
        Ok(())
    }
}

impl Default for CommandStorage {
    fn default() -> Self {
        Self::new().expect("Failed to create default CommandStorage")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn temp_storage() -> (CommandStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage_path = temp_dir.path().join("commands.json");
        let storage = CommandStorage::with_path(storage_path).unwrap();
        (storage, temp_dir)
    }

    #[test]
    fn test_create_and_read() {
        let (storage, _temp) = temp_storage();
        
        let cmd = Command::new("Test Command".to_string(), "echo".to_string())
            .with_args(vec!["hello".to_string()]);
        
        let created = storage.create(cmd.clone()).unwrap();
        assert_eq!(created.name, "Test Command");
        
        let read = storage.read(created.id).unwrap();
        assert_eq!(read.name, "Test Command");
        assert_eq!(read.command, "echo");
    }

    #[test]
    fn test_duplicate_name() {
        let (storage, _temp) = temp_storage();
        
        let cmd1 = Command::new("Duplicate".to_string(), "echo".to_string());
        let cmd2 = Command::new("Duplicate".to_string(), "ls".to_string());
        
        storage.create(cmd1).unwrap();
        let result = storage.create(cmd2);
        
        assert!(matches!(result, Err(CommandArgusError::DuplicateName(_))));
    }

    #[test]
    fn test_update() {
        let (storage, _temp) = temp_storage();
        
        let cmd = Command::new("Original".to_string(), "echo".to_string());
        let created = storage.create(cmd).unwrap();
        
        let updated = storage.update(created.id, |c| {
            c.name = "Updated".to_string();
            c.add_tag("test".to_string());
        }).unwrap();
        
        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.tags, vec!["test"]);
    }

    #[test]
    fn test_delete() {
        let (storage, _temp) = temp_storage();
        
        let cmd = Command::new("To Delete".to_string(), "echo".to_string());
        let created = storage.create(cmd).unwrap();
        
        storage.delete(created.id).unwrap();
        
        let result = storage.read(created.id);
        assert!(matches!(result, Err(CommandArgusError::NotFound(_))));
    }

    #[test]
    fn test_list_and_search() {
        let (storage, _temp) = temp_storage();
        
        let cmd1 = Command::new("First Command".to_string(), "echo".to_string());
        let mut cmd2 = Command::new("Second Command".to_string(), "ls".to_string());
        cmd2.add_tag("filesystem".to_string());
        
        storage.create(cmd1).unwrap();
        storage.create(cmd2).unwrap();
        
        let all = storage.list().unwrap();
        assert_eq!(all.len(), 2);
        
        let by_name = storage.search_by_name("First").unwrap();
        assert_eq!(by_name.len(), 1);
        assert_eq!(by_name[0].name, "First Command");
        
        let by_tag = storage.search_by_tags(&["filesystem".to_string()]).unwrap();
        assert_eq!(by_tag.len(), 1);
        assert_eq!(by_tag[0].name, "Second Command");
    }
}