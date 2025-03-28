/* eslint-disable @typescript-eslint/no-unused-vars */

import { createContext, useContext, useState, useEffect } from "react";
// import { createFormHook, createFormHookContexts } from "@tanstack/react-form";
import WidgetForm from "./form";

// import "./App.css";

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
  return (
    <div className="min-h-screen flex justify-center p-4">
      <WidgetForm />
    </div>
  );
};

export default App;
