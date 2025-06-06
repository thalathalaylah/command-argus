import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Command, CreateCommandRequest, UpdateCommandRequest, EnvironmentVariable } from '../types';

interface CommandFormProps {
  editingCommand?: Command | null;
  onSave: () => void;
  onCancel: () => void;
}

export function CommandForm({ editingCommand, onSave, onCancel }: CommandFormProps) {
  const [name, setName] = useState('');
  const [command, setCommand] = useState('');
  const [args, setArgs] = useState('');
  const [description, setDescription] = useState('');
  const [workingDirectory, setWorkingDirectory] = useState('');
  const [tags, setTags] = useState('');
  const [envVars, setEnvVars] = useState<EnvironmentVariable[]>([]);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (editingCommand) {
      setName(editingCommand.name);
      setCommand(editingCommand.command);
      setArgs(editingCommand.args.join(' '));
      setDescription(editingCommand.description || '');
      setWorkingDirectory(editingCommand.working_directory || '');
      setTags(editingCommand.tags.join(', '));
      setEnvVars(editingCommand.environment_variables);
    } else {
      // Reset form for new command
      setName('');
      setCommand('');
      setArgs('');
      setDescription('');
      setWorkingDirectory('');
      setTags('');
      setEnvVars([]);
    }
  }, [editingCommand]);

  const handleAddEnvVar = () => {
    setEnvVars([...envVars, { key: '', value: '' }]);
  };

  const handleUpdateEnvVar = (index: number, field: 'key' | 'value', value: string) => {
    const updated = [...envVars];
    updated[index] = { ...updated[index], [field]: value };
    setEnvVars(updated);
  };

  const handleRemoveEnvVar = (index: number) => {
    setEnvVars(envVars.filter((_, i) => i !== index));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!name.trim() || !command.trim()) {
      alert('Name and command are required!');
      return;
    }

    setSaving(true);

    try {
      const argsArray = args.trim() ? args.split(' ').filter(a => a) : [];
      const tagsArray = tags.trim() ? tags.split(',').map(t => t.trim()).filter(t => t) : [];
      const validEnvVars = envVars.filter(ev => ev.key && ev.value);

      if (editingCommand) {
        const updateRequest: UpdateCommandRequest = {
          name,
          command,
          args: argsArray,
          description: description || undefined,
          working_directory: workingDirectory || undefined,
          environment_variables: validEnvVars,
          tags: tagsArray
        };
        
        await invoke('update_command', {
          id: editingCommand.id,
          request: updateRequest
        });
      } else {
        const createRequest: CreateCommandRequest = {
          name,
          command,
          args: argsArray,
          description: description || undefined,
          working_directory: workingDirectory || undefined,
          environment_variables: validEnvVars,
          tags: tagsArray
        };
        
        await invoke('create_command', { request: createRequest });
      }
      
      onSave();
    } catch (err) {
      alert(`Failed to save command: ${err}`);
    } finally {
      setSaving(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <h2 className="text-xl font-bold mb-4">
        {editingCommand ? 'Edit Command' : 'Create New Command'}
      </h2>

      <div>
        <label className="block text-sm font-medium mb-1">Name *</label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          required
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">Command *</label>
        <input
          type="text"
          value={command}
          onChange={(e) => setCommand(e.target.value)}
          required
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">Arguments</label>
        <input
          type="text"
          value={args}
          onChange={(e) => setArgs(e.target.value)}
          placeholder="Space-separated arguments"
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">Description</label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          rows={3}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">Working Directory</label>
        <input
          type="text"
          value={workingDirectory}
          onChange={(e) => setWorkingDirectory(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <label className="block text-sm font-medium mb-1">Tags</label>
        <input
          type="text"
          value={tags}
          onChange={(e) => setTags(e.target.value)}
          placeholder="Comma-separated tags"
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
      </div>

      <div>
        <div className="flex justify-between items-center mb-2">
          <label className="text-sm font-medium">Environment Variables</label>
          <button
            type="button"
            onClick={handleAddEnvVar}
            className="px-3 py-1 text-sm bg-green-500 text-white rounded hover:bg-green-600"
          >
            Add Variable
          </button>
        </div>
        
        {envVars.map((envVar, index) => (
          <div key={index} className="flex gap-2 mb-2">
            <input
              type="text"
              value={envVar.key}
              onChange={(e) => handleUpdateEnvVar(index, 'key', e.target.value)}
              placeholder="Key"
              className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <input
              type="text"
              value={envVar.value}
              onChange={(e) => handleUpdateEnvVar(index, 'value', e.target.value)}
              placeholder="Value"
              className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button
              type="button"
              onClick={() => handleRemoveEnvVar(index)}
              className="px-3 py-2 bg-red-500 text-white rounded hover:bg-red-600"
            >
              Remove
            </button>
          </div>
        ))}
      </div>

      <div className="flex gap-2 pt-4">
        <button
          type="submit"
          disabled={saving}
          className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
        >
          {saving ? 'Saving...' : (editingCommand ? 'Update' : 'Create')}
        </button>
        <button
          type="button"
          onClick={onCancel}
          disabled={saving}
          className="px-4 py-2 bg-gray-500 text-white rounded-md hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500"
        >
          Cancel
        </button>
      </div>
    </form>
  );
}