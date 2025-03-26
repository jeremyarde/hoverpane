/* eslint-disable @typescript-eslint/no-unused-vars */

import { createContext, useContext, useState, useEffect } from "react";
// import { createFormHook, createFormHookContexts } from "@tanstack/react-form";
import WidgetForm from "./form";

import "./App.css";

interface RustMessage {
  data_key: string;
  window_id: string;
  message: string; // json string
  timestamp: string;
}

// Update the Message type
type Message = RustMessage;

// Declare the global handler
declare global {
  interface Window {
    onRustMessage: (element: string) => void;
    ipc: {
      postMessage: (message: string) => void;
    };
    WIDGET_ID: string;
    WINDOW_ID: string;
  }
}

const App = () => {
  const [bgColor, setBgColor] = useState<[number, number, number, number]>([
    245, 246, 247, 1,
  ]);

  return (
    <div
      className="min-h-screen transition-colors duration-200 flex items-center justify-center p-4"
      style={{
        backgroundColor: `rgba(${bgColor.join(",")})`,
      }}
    >
      <div className="w-full max-w-[420px] bg-white rounded-xl shadow-lg overflow-hidden">
        <div className="bg-gray-50 px-6 py-4 border-b border-gray-100">
          <h1 className="text-lg font-semibold text-gray-900">Create Widget</h1>
          <p className="text-sm text-gray-500 mt-1">
            Configure your widget settings below
          </p>
        </div>
        <WidgetForm onBackgroundColorChange={setBgColor} />
      </div>
    </div>
  );
};

export default App;
