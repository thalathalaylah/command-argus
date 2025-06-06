use command_argus_logic::{Command, CommandStorage, EnvironmentVariable};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

// State to hold the CommandStorage instance
struct AppState {
    storage: Mutex<CommandStorage>,
}

// DTOs for frontend communication
#[derive(Serialize, Deserialize)]
struct CommandDto {
    id: String,
    name: String,
    command: String,
    args: Vec<String>,
    description: Option<String>,
    working_directory: Option<String>,
    environment_variables: Vec<EnvironmentVariableDto>,
    tags: Vec<String>,
    created_at: String,
    updated_at: String,
    last_used_at: Option<String>,
    use_count: u32,
}

#[derive(Serialize, Deserialize)]
struct EnvironmentVariableDto {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct CreateCommandRequest {
    name: String,
    command: String,
    args: Vec<String>,
    description: Option<String>,
    working_directory: Option<String>,
    environment_variables: Vec<EnvironmentVariableDto>,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UpdateCommandRequest {
    name: Option<String>,
    command: Option<String>,
    args: Option<Vec<String>>,
    description: Option<String>,
    working_directory: Option<String>,
    environment_variables: Option<Vec<EnvironmentVariableDto>>,
    tags: Option<Vec<String>>,
}

// Convert Command to CommandDto
fn command_to_dto(cmd: &Command) -> CommandDto {
    CommandDto {
        id: cmd.id.to_string(),
        name: cmd.name.clone(),
        command: cmd.command.clone(),
        args: cmd.args.clone(),
        description: cmd.description.clone(),
        working_directory: cmd.working_directory.clone(),
        environment_variables: cmd.environment_variables
            .iter()
            .map(|ev| EnvironmentVariableDto {
                key: ev.key.clone(),
                value: ev.value.clone(),
            })
            .collect(),
        tags: cmd.tags.clone(),
        created_at: cmd.created_at.to_rfc3339(),
        updated_at: cmd.updated_at.to_rfc3339(),
        last_used_at: cmd.last_used_at.map(|dt| dt.to_rfc3339()),
        use_count: cmd.use_count,
    }
}

// Tauri commands
#[tauri::command]
fn list_commands(state: State<AppState>) -> Result<Vec<CommandDto>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.list()
        .map(|commands| commands.into_iter().map(|cmd| command_to_dto(&cmd)).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_command(id: String, state: State<AppState>) -> Result<CommandDto, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.read(uuid)
        .map(|cmd| command_to_dto(&cmd))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn create_command(request: CreateCommandRequest, state: State<AppState>) -> Result<CommandDto, String> {
    let mut cmd = Command::new(request.name, request.command)
        .with_args(request.args);
    
    if let Some(desc) = request.description {
        cmd = cmd.with_description(desc);
    }
    
    if let Some(wd) = request.working_directory {
        cmd = cmd.with_working_directory(wd);
    }
    
    for env_var in request.environment_variables {
        cmd.add_environment_variable(env_var.key, env_var.value);
    }
    
    for tag in request.tags {
        cmd.add_tag(tag);
    }
    
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.create(cmd)
        .map(|created_cmd| command_to_dto(&created_cmd))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn update_command(id: String, request: UpdateCommandRequest, state: State<AppState>) -> Result<CommandDto, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    
    storage.update(uuid, |cmd| {
        if let Some(name) = &request.name {
            cmd.name = name.clone();
        }
        if let Some(command) = &request.command {
            cmd.command = command.clone();
        }
        if let Some(args) = &request.args {
            cmd.args = args.clone();
        }
        if let Some(description) = &request.description {
            cmd.description = Some(description.clone());
        }
        if let Some(working_directory) = &request.working_directory {
            cmd.working_directory = Some(working_directory.clone());
        }
        if let Some(env_vars) = &request.environment_variables {
            cmd.environment_variables = env_vars.iter()
                .map(|ev| EnvironmentVariable {
                    key: ev.key.clone(),
                    value: ev.value.clone(),
                })
                .collect();
        }
        if let Some(tags) = &request.tags {
            cmd.tags = tags.clone();
        }
        cmd.update();
    })
    .and_then(|_| storage.read(uuid))
    .map(|cmd| command_to_dto(&cmd))
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_command(id: String, state: State<AppState>) -> Result<(), String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.delete(uuid).map_err(|e| e.to_string())
}

#[tauri::command]
fn search_commands_by_name(query: String, state: State<AppState>) -> Result<Vec<CommandDto>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.search_by_name(&query)
        .map(|commands| commands.into_iter().map(|cmd| command_to_dto(&cmd)).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn search_commands_by_tags(tags: Vec<String>, state: State<AppState>) -> Result<Vec<CommandDto>, String> {
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    storage.search_by_tags(&tags)
        .map(|commands| commands.into_iter().map(|cmd| command_to_dto(&cmd)).collect())
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        storage: Mutex::new(CommandStorage::new().expect("Failed to initialize storage")),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_commands,
            get_command,
            create_command,
            update_command,
            delete_command,
            search_commands_by_name,
            search_commands_by_tags
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
