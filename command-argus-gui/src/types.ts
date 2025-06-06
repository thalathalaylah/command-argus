export interface EnvironmentVariable {
  key: string;
  value: string;
}

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
}

export interface CreateCommandRequest {
  name: string;
  command: string;
  args: string[];
  description?: string;
  working_directory?: string;
  environment_variables: EnvironmentVariable[];
  tags: string[];
}

export interface UpdateCommandRequest {
  name?: string;
  command?: string;
  args?: string[];
  description?: string;
  working_directory?: string;
  environment_variables?: EnvironmentVariable[];
  tags?: string[];
}

export interface ExecutionResult {
  stdout: string;
  stderr: string;
  exit_code: number;
  success: boolean;
}