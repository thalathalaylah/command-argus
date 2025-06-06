import { useState } from "react";
import { CommandList } from "./components/CommandList";
import { CommandForm } from "./components/CommandForm";
import { Command } from "./types";
import "./App.css";

function App() {
  const [showForm, setShowForm] = useState(false);
  const [editingCommand, setEditingCommand] = useState<Command | null>(null);
  const [refreshTrigger, setRefreshTrigger] = useState(0);

  const handleNewCommand = () => {
    setEditingCommand(null);
    setShowForm(true);
  };

  const handleEditCommand = (command: Command) => {
    setEditingCommand(command);
    setShowForm(true);
  };

  const handleSaveCommand = () => {
    setShowForm(false);
    setEditingCommand(null);
    setRefreshTrigger(prev => prev + 1);
  };

  const handleCancel = () => {
    setShowForm(false);
    setEditingCommand(null);
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-6xl mx-auto p-6">
        <div className="bg-white rounded-lg shadow-md p-6">
          <div className="flex justify-between items-center mb-6">
            <h1 className="text-3xl font-bold text-gray-800">Command Argus</h1>
            {!showForm && (
              <button
                onClick={handleNewCommand}
                className="px-4 py-2 bg-green-500 text-white rounded-md hover:bg-green-600 focus:outline-none focus:ring-2 focus:ring-green-500"
              >
                New Command
              </button>
            )}
          </div>

          {showForm ? (
            <CommandForm
              editingCommand={editingCommand}
              onSave={handleSaveCommand}
              onCancel={handleCancel}
            />
          ) : (
            <CommandList
              onEdit={handleEditCommand}
              refreshTrigger={refreshTrigger}
            />
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
