import React, { useState } from 'react';
import { CommandParameter } from '../types';

interface ParameterInputModalProps {
  parameters: CommandParameter[];
  onSubmit: (values: Record<string, string>) => void;
  onCancel: () => void;
}

export const ParameterInputModal: React.FC<ParameterInputModalProps> = ({
  parameters,
  onSubmit,
  onCancel,
}) => {
  const [values, setValues] = useState<Record<string, string>>(() => {
    const initial: Record<string, string> = {};
    parameters.forEach(param => {
      initial[param.name] = param.default_value || '';
    });
    return initial;
  });

  const [errors, setErrors] = useState<Record<string, string>>({});

  const handleChange = (name: string, value: string) => {
    setValues(prev => ({ ...prev, [name]: value }));
    // Clear error when user starts typing
    if (errors[name]) {
      setErrors(prev => {
        const newErrors = { ...prev };
        delete newErrors[name];
        return newErrors;
      });
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    // Validate required fields
    const newErrors: Record<string, string> = {};
    parameters.forEach(param => {
      if (param.required && !values[param.name]?.trim()) {
        newErrors[param.name] = `${param.placeholder} is required`;
      }
    });

    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors);
      return;
    }

    onSubmit(values);
  };

  const renderInput = (param: CommandParameter) => {
    const value = values[param.name] || '';
    const error = errors[param.name];

    switch (param.parameter_type) {
      case 'select':
        return (
          <select
            value={value}
            onChange={(e) => handleChange(param.name, e.target.value)}
            className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
              error ? 'border-red-500' : 'border-gray-300'
            }`}
            required={param.required}
          >
            <option value="">Select {param.placeholder}</option>
            {param.options?.map(option => (
              <option key={option} value={option}>
                {option}
              </option>
            ))}
          </select>
        );

      case 'file':
      case 'directory':
        return (
          <div className="flex gap-2">
            <input
              type="text"
              value={value}
              onChange={(e) => handleChange(param.name, e.target.value)}
              placeholder={`Path to ${param.parameter_type}`}
              className={`flex-1 px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
                error ? 'border-red-500' : 'border-gray-300'
              }`}
              required={param.required}
            />
            <button
              type="button"
              className="px-3 py-2 text-sm bg-gray-100 hover:bg-gray-200 rounded-md border border-gray-300"
              onClick={() => {
                // TODO: Implement file/directory picker using Tauri's dialog API
                console.log(`Open ${param.parameter_type} picker`);
              }}
            >
              Browse
            </button>
          </div>
        );

      case 'text':
      default:
        return (
          <input
            type="text"
            value={value}
            onChange={(e) => handleChange(param.name, e.target.value)}
            placeholder={param.placeholder}
            className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
              error ? 'border-red-500' : 'border-gray-300'
            }`}
            required={param.required}
          />
        );
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 w-full max-w-md max-h-[80vh] overflow-y-auto">
        <h2 className="text-xl font-bold mb-4">Enter Parameters</h2>
        
        <form onSubmit={handleSubmit} className="space-y-4">
          {parameters.map(param => (
            <div key={param.name}>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                {param.placeholder}
                {param.required && <span className="text-red-500 ml-1">*</span>}
              </label>
              {renderInput(param)}
              {errors[param.name] && (
                <p className="mt-1 text-sm text-red-600">{errors[param.name]}</p>
              )}
            </div>
          ))}

          <div className="flex gap-2 mt-6">
            <button
              type="submit"
              className="flex-1 bg-blue-500 text-white px-4 py-2 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              Execute
            </button>
            <button
              type="button"
              onClick={onCancel}
              className="flex-1 bg-gray-300 text-gray-700 px-4 py-2 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500"
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};