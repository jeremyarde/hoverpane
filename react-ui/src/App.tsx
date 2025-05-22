/* eslint-disable @typescript-eslint/no-unused-vars */

import { useState } from "react";
import EditWidgets from "./EditWidgets";
import DataWidget from "./DataWidget";
import SettingsWidget from "./SettingsWidget";
import SimpleCreateWidgetForm from "./SimpleCreateWidget";
// import { Checkout } from "./Checkout";

interface RustMessage {
  data_key: string;
  window_id: string;
  message: string; // json string
  timestamp: string;
}

type Message = RustMessage;

declare global {
  interface Window {
    onRustMessage: (element: string) => void;
    ipc: {
      postMessage: (message: string) => void;
    };
    WIDGET_ID: string;
    WINDOW_ID: string;
    PORT: string;
  }
}

type Component = "create" | "edit" | "data" | "settings";

const App = () => {
  const [displayedWidget, setDisplayedWidget] = useState<Component>("create");
  return (
    <div className="flex justify-center p-4 min-h-screen">
      <div className="flex flex-col min-w-[400px]">
        <div className="flex flex-row">
          <button
            className={`flex-1 relative px-4 py-0.5 font-bold text-sm uppercase border-1 rounded-t-lg transition-colors ${
              displayedWidget === "create"
                ? "bg-[#98EECC] hover:bg-[#7DCCAA] border-b-0 after:absolute after:bottom-[-2px] after:left-0 after:right-0 after:h-[2px] after:bg-[#98EECC]"
                : "bg-white hover:bg-gray-100"
            }`}
            onClick={() => setDisplayedWidget("create")}
          >
            Create
          </button>
          <button
            className={`flex-1 relative px-4 py-0.5 font-bold text-sm uppercase border-1 rounded-t-lg transition-colors ${
              displayedWidget === "edit"
                ? "bg-[#98EECC] hover:bg-[#7DCCAA] border-b-0 after:absolute after:bottom-[-2px] after:left-0 after:right-0 after:h-[2px] after:bg-[#98EECC]"
                : "bg-white hover:bg-gray-100"
            }`}
            onClick={() => setDisplayedWidget("edit")}
          >
            Edit
          </button>
          {/* <button
            className={`flex-1 relative px-4 py-0.5 font-bold text-sm uppercase border-1 rounded-t-lg transition-colors ${
              displayedWidget === "data"
                ? "bg-[#98EECC] hover:bg-[#7DCCAA] border-b-0 after:absolute after:bottom-[-2px] after:left-0 after:right-0 after:h-[2px] after:bg-[#98EECC]"
                : "bg-white hover:bg-gray-100"
            }`}
            onClick={() => setDisplayedWidget("data")}
          >
            Data
          </button> */}
          <button
            className={`flex-1 relative px-4 py-0.5 font-bold text-sm uppercase border-1 rounded-t-lg transition-colors ${
              displayedWidget === "settings"
                ? "bg-[#98EECC] hover:bg-[#7DCCAA] border-b-0 after:absolute after:bottom-[-2px] after:left-0 after:right-0 after:h-[2px] after:bg-[#98EECC]"
                : "bg-white hover:bg-gray-100"
            }`}
            onClick={() => setDisplayedWidget("settings")}
          >
            Settings
          </button>
        </div>
        {/* Border around the widget contents */}
        <div className="border-1 border-black rounded-b-lg min-h-[200px]">
          {displayedWidget === "create" && <SimpleCreateWidgetForm />}
          {displayedWidget === "edit" && <EditWidgets />}
          {displayedWidget === "data" && <DataWidget />}
          {displayedWidget === "settings" && (
            <div className="p-4">
              <SettingsWidget />
              {/* <div className="mt-8">
                <Checkout
                  onSuccess={() => console.log("Checkout successful")}
                  onError={(error) => console.error("Checkout error:", error)}
                />
              </div> */}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default App;
