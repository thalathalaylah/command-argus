import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Command, ExecutionResult } from '../types';

interface CommandListProps {
  onEdit: (command: Command) => void;
  refreshTrigger?: number;
}

export function CommandList({ onEdit, refreshTrigger }: CommandListProps) {
  const [commands, setCommands] = useState<Command[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [executingCommands, setExecutingCommands] = useState<Set<string>>(new Set());
  const [executionResults, setExecutionResults] = useState<Map<string, ExecutionResult>>(new Map());

  const loadCommands = async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await invoke<Command[]>('list_commands');
      setCommands(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load commands');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCommands();
  }, [refreshTrigger]);

  const handleDelete = async (id: string) => {
    if (!confirm('Are you sure you want to delete this command?')) {
      return;
    }

    try {
      await invoke('delete_command', { id });
      await loadCommands();
    } catch (err) {
      alert(`Failed to delete command: ${err}`);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      await loadCommands();
      return;
    }

    try {
      setLoading(true);
      const result = await invoke<Command[]>('search_commands_by_name', {
        query: searchQuery
      });
      setCommands(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Search failed');
    } finally {
      setLoading(false);
    }
  };

  const handleExecute = async (commandId: string, useShell: boolean = true) => {
    setExecutingCommands(prev => new Set(prev).add(commandId));
    
    try {
      const result = await invoke<ExecutionResult>('execute_command', {
        id: commandId,
        useShell
      });
      
      setExecutionResults(prev => new Map(prev).set(commandId, result));
      
      // Reload commands to update use count
      await loadCommands();
    } catch (err) {
      const errorResult: ExecutionResult = {
        stdout: '',
        stderr: err instanceof Error ? err.message : 'Command execution failed',
        exit_code: -1,
        success: false
      };
      setExecutionResults(prev => new Map(prev).set(commandId, errorResult));
    } finally {
      setExecutingCommands(prev => {
        const newSet = new Set(prev);
        newSet.delete(commandId);
        return newSet;
      });
    }
  };

  const clearExecutionResult = (commandId: string) => {
    setExecutionResults(prev => {
      const newMap = new Map(prev);
      newMap.delete(commandId);
      return newMap;
    });
  };

  if (loading) {
    return <div className="text-center py-8">Loading commands...</div>;
  }

  if (error) {
    return <div className="text-red-500 text-center py-8">Error: {error}</div>;
  }

  return (
    <div className="space-y-4">
      <div className="flex gap-2">
        <input
          type="text"
          placeholder="Search commands..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <button
          onClick={handleSearch}
          className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          Search
        </button>
        <button
          onClick={loadCommands}
          className="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          Clear
        </button>
      </div>

      {commands.length === 0 ? (
        <div className="text-center py-8 text-gray-500">
          No commands found. Create your first command!
        </div>
      ) : (
        <div className="grid gap-4">
          {commands.map((command) => (
            <div
              key={command.id}
              className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
            >
              <div className="flex justify-between items-start mb-2">
                <h3 className="text-lg font-semibold">{command.name}</h3>
                <div className="flex gap-2">
                  <button
                    onClick={() => handleExecute(command.id)}
                    disabled={executingCommands.has(command.id)}
                    className="px-3 py-1 text-sm bg-green-500 text-white rounded hover:bg-green-600 disabled:bg-gray-400"
                  >
                    {executingCommands.has(command.id) ? 'Running...' : 'Run'}
                  </button>
                  <button
                    onClick={() => onEdit(command)}
                    className="px-3 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600"
                  >
                    Edit
                  </button>
                  <button
                    onClick={() => handleDelete(command.id)}
                    className="px-3 py-1 text-sm bg-red-500 text-white rounded hover:bg-red-600"
                  >
                    Delete
                  </button>
                </div>
              </div>
              
              <div className="space-y-2 text-sm">
                <div className="font-mono bg-gray-100 p-2 rounded">
                  {command.command} {command.args.join(' ')}
                </div>
                
                {command.description && (
                  <p className="text-gray-600">{command.description}</p>
                )}
                
                {command.tags.length > 0 && (
                  <div className="flex gap-2">
                    {command.tags.map((tag) => (
                      <span
                        key={tag}
                        className="px-2 py-1 bg-gray-200 text-gray-700 rounded-md text-xs"
                      >
                        {tag}
                      </span>
                    ))}
                  </div>
                )}
                
                <div className="text-gray-500 text-xs">
                  Used {command.use_count} times
                  {command.last_used_at && ` • Last used: ${new Date(command.last_used_at).toLocaleDateString()}`}
                </div>
                
                {executionResults.has(command.id) && (
                  <div className="mt-3 border-t pt-3">
                    <div className="flex justify-between items-center mb-2">
                      <h4 className="font-semibold text-sm">Execution Result:</h4>
                      <button
                        onClick={() => clearExecutionResult(command.id)}
                        className="text-xs text-gray-500 hover:text-gray-700"
                      >
                        Clear
                      </button>
                    </div>
                    {(() => {
                      const result = executionResults.get(command.id)!;
                      return (
                        <div className={`text-xs space-y-2 ${result.success ? '' : 'text-red-600'}`}>
                          {result.stdout && (
                            <div>
                              <div className="font-semibold">Output:</div>
                              <pre className="bg-gray-100 p-2 rounded overflow-x-auto">{result.stdout}</pre>
                            </div>
                          )}
                          {result.stderr && (
                            <div>
                              <div className="font-semibold">Error:</div>
                              <pre className="bg-red-50 p-2 rounded overflow-x-auto">{result.stderr}</pre>
                            </div>
                          )}
                          <div className="text-gray-500">
                            Exit code: {result.exit_code} • Status: {result.success ? 'Success' : 'Failed'}
                          </div>
                        </div>
                      );
                    })()}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}