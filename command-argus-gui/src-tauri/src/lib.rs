use command_argus_logic::{Command, CommandStorage, EnvironmentVariable, CommandExecutor, CommandParameter, ParameterType};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use tauri::State;
use uuid::Uuid;

// State to hold the CommandStorage instance
struct AppState {
    storage: Mutex<CommandStorage>,
    executor: CommandExecutor,
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
    parameters: Vec<CommandParameterDto>,
}

#[derive(Serialize, Deserialize)]
struct EnvironmentVariableDto {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct CommandParameterDto {
    name: String,
    placeholder: String,
    parameter_type: String,
    required: bool,
    default_value: Option<String>,
    options: Option<Vec<String>>,
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
    parameters: Vec<CommandParameterDto>,
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
    parameters: Option<Vec<CommandParameterDto>>,
}

#[derive(Serialize, Deserialize)]
struct ExecutionResultDto {
    stdout: String,
    stderr: String,
    exit_code: i32,
    success: bool,
}

// Convert ParameterType to string
fn parameter_type_to_string(param_type: &ParameterType) -> String {
    match param_type {
        ParameterType::Text => "text".to_string(),
        ParameterType::File => "file".to_string(),
        ParameterType::Directory => "directory".to_string(),
        ParameterType::Select => "select".to_string(),
    }
}

// Convert string to ParameterType
fn string_to_parameter_type(s: &str) -> ParameterType {
    match s {
        "file" => ParameterType::File,
        "directory" => ParameterType::Directory,
        "select" => ParameterType::Select,
        _ => ParameterType::Text,
    }
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
        parameters: cmd.parameters
            .iter()
            .map(|p| CommandParameterDto {
                name: p.name.clone(),
                placeholder: p.placeholder.clone(),
                parameter_type: parameter_type_to_string(&p.parameter_type),
                required: p.required,
                default_value: p.default_value.clone(),
                options: p.options.clone(),
            })
            .collect(),
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
    
    for param_dto in request.parameters {
        cmd.add_parameter(CommandParameter {
            name: param_dto.name,
            placeholder: param_dto.placeholder,
            parameter_type: string_to_parameter_type(&param_dto.parameter_type),
            required: param_dto.required,
            default_value: param_dto.default_value,
            options: param_dto.options,
        });
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
        if let Some(parameters) = &request.parameters {
            cmd.parameters = parameters.iter()
                .map(|p| CommandParameter {
                    name: p.name.clone(),
                    placeholder: p.placeholder.clone(),
                    parameter_type: string_to_parameter_type(&p.parameter_type),
                    required: p.required,
                    default_value: p.default_value.clone(),
                    options: p.options.clone(),
                })
                .collect();
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

#[tauri::command]
fn execute_command(id: String, use_shell: bool, state: State<AppState>) -> Result<ExecutionResultDto, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    // Get the command and mark it as used
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    let command = storage.read(uuid).map_err(|e| e.to_string())?;
    
    // Mark the command as used
    storage.update(uuid, |cmd| {
        cmd.mark_as_used();
    }).map_err(|e| e.to_string())?;
    
    // Execute the command
    let result = if use_shell {
        state.executor.execute_with_shell(&command)
    } else {
        state.executor.execute(&command)
    };
    
    result
        .map(|exec_result| ExecutionResultDto {
            stdout: exec_result.stdout,
            stderr: exec_result.stderr,
            exit_code: exec_result.exit_code,
            success: exec_result.success,
        })
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn execute_command_with_parameters(
    id: String,
    parameters: HashMap<String, String>,
    use_shell: bool,
    state: State<AppState>
) -> Result<ExecutionResultDto, String> {
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    // Get the command and mark it as used
    let storage = state.storage.lock().map_err(|e| e.to_string())?;
    let mut command = storage.read(uuid).map_err(|e| e.to_string())?;
    
    // Replace placeholders with parameter values
    let (new_command, new_args) = command.replace_placeholders(&parameters);
    command.command = new_command;
    command.args = new_args;
    
    // Mark the command as used
    storage.update(uuid, |cmd| {
        cmd.mark_as_used();
    }).map_err(|e| e.to_string())?;
    
    // Execute the command with replaced parameters
    let result = if use_shell {
        state.executor.execute_with_shell(&command)
    } else {
        state.executor.execute(&command)
    };
    
    result
        .map(|exec_result| ExecutionResultDto {
            stdout: exec_result.stdout,
            stderr: exec_result.stderr,
            exit_code: exec_result.exit_code,
            success: exec_result.success,
        })
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        storage: Mutex::new(CommandStorage::new().expect("Failed to initialize storage")),
        executor: CommandExecutor::new(),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            list_commands,
            get_command,
            create_command,
            update_command,
            delete_command,
            search_commands_by_name,
            search_commands_by_tags,
            execute_command,
            execute_command_with_parameters
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
