export interface EnvironmentVariable {
  key: string;
  value: string;
}

export interface CommandParameter {
  name: string;
  placeholder: string;
  parameter_type: ParameterType;
  required: boolean;
  default_value?: string;
  options?: string[];
}

export type ParameterType = 'text' | 'file' | 'directory' | 'select';

export interface Command {
  id: string;
  name: string;
  command: string;
  args: string[];
  description?: string;
  working_directory?: string;
  environment_variables: EnvironmentVariable[];
  tags: string[];
  created_at: string;
  updated_at: string;
  last_used_at?: string;
  use_count: number;
  parameters: CommandParameter[];
  mise_enabled: boolean;
}

export interface CreateCommandRequest {
  name: string;
  command: string;
  args: string[];
  description?: string;
  working_directory?: string;
  environment_variables: EnvironmentVariable[];
  tags: string[];
  parameters: CommandParameter[];
  mise_enabled?: boolean;
}

export interface UpdateCommandRequest {
  name?: string;
  command?: string;
  args?: string[];
  description?: string;
  working_directory?: string;
  environment_variables?: EnvironmentVariable[];
  tags?: string[];
  parameters?: CommandParameter[];
  mise_enabled?: boolean;
}

export interface ExecutionResult {
  stdout: string;
  stderr: string;
  exit_code: number;
  success: boolean;
}