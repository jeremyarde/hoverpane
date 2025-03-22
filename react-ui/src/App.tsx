/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable @typescript-eslint/no-explicit-any */

import { createContext, useContext, useState } from "react";
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
  }
}

// Create a context for the message handler
const MessageContext = createContext<{
  messages: Message[];
  addMessage: (message: Message) => void;
}>({
  messages: [],
  addMessage: () => {},
});

// Custom hook to use the message context
export const useMessages = () => {
  return useContext(MessageContext);
};

interface NewWidgetConfig {
  title: string;
  widget_type: string;
  url: string;
  selector: string;
  refresh_interval: number;
}

// Add validation methods
interface NewWidgetConfigValidation {
  isValid(config: NewWidgetConfig): boolean;
  getErrors(config: NewWidgetConfig): string[];
}

function isValidUrl(url: string) {
  try {
    new URL(url);
    return true;
  } catch (error) {
    console.error("Invalid URL format", error);
    return false;
  }
}

const widgetConfigValidation: NewWidgetConfigValidation = {
  isValid(config: NewWidgetConfig): boolean {
    return !this.getErrors(config).length;
  },

  getErrors(config: NewWidgetConfig): string[] {
    const errors: string[] = [];

    if (!config.url.trim()) {
      errors.push("URL is required");
    } else if (!isValidUrl(config.url)) {
      errors.push("Invalid URL format");
    }

    if (!config.selector.trim()) {
      errors.push("Selector is required");
    }

    if (config.refresh_interval && config.refresh_interval < 1000) {
      errors.push("Refresh interval must be at least 1000ms");
    }

    return errors;
  },
};

// Usage in form
const NewWidgetForm: React.FC = () => {
  const [config, setConfig] = useState<NewWidgetConfig>({
    title: "",
    widget_type: "",
    url: "",
    selector: "",
    refresh_interval: 1000,
  });
  const [errors, setErrors] = useState<string[]>([]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // const validationErrors = widgetConfigValidation.getErrors(config);
    // if (validationErrors.length) {
    //   setErrors(validationErrors);
    //   return;
    // }
    const message = {
      createwidget: {
        title: config.title,
        widget_type: {
          source: {
            url: config.url,
            element_selectors: [config.selector],
            refresh_interval: config.refresh_interval,
          },
        },
      },
    };
    console.log("Creating a widget: ", message);
    window.ipc.postMessage(JSON.stringify(message));
  };

  return (
    <form onSubmit={handleSubmit} className="newview-form">
      {errors.map((error) => (
        <div key={error} className="error">
          {error}
        </div>
      ))}
      <input
        className="newview-form-input"
        placeholder="Title"
        value={config.title}
        onChange={(e) => setConfig({ ...config, title: e.target.value })}
      />
      <select
        className="newview-form-input"
        required
        value={config.widget_type}
        onChange={(e) => setConfig({ ...config, widget_type: e.target.value })}
      >
        <option value="source">Source</option>
        <option value="display">Display</option>
        <option value="tracker">Tracker</option>
        <option value="controls">Controls</option>
      </select>
      <input
        className="newview-form-input"
        placeholder="URL"
        value={config.url}
        onChange={(e) => setConfig({ ...config, url: e.target.value })}
      />
      <input
        className="newview-form-input"
        placeholder="Selector"
        value={config.selector}
        onChange={(e) => setConfig({ ...config, selector: e.target.value })}
      />
      <input
        className="newview-form-input"
        placeholder="Refresh Interval"
        value={config.refresh_interval}
        onChange={(e) =>
          setConfig({ ...config, refresh_interval: parseInt(e.target.value) })
        }
      />

      <button type="submit">Create View</button>
    </form>
  );
};

const handleCopyPaste = (e: KeyboardEvent) => {
  if (e.metaKey && e.key === "v") {
    e.preventDefault();
    // const text = e.clipboardData.getData("text/plain");
    // document.execCommand("insertText", false, text);
    console.log("paste");
    // insert text into the active element at the cursor position
    const activeElement = document.activeElement as HTMLElement;
    if (activeElement) {
      const selection = window.getSelection();
      if (selection) {
        const range = document.createRange();
        range.selectNodeContents(activeElement);
        range.collapse(false);
        selection.removeAllRanges();
        selection.addRange(range);
        // const text = "jeremy was here";
        // selection.insertNode(document.createTextNode(text));
      }
    }
  }
  if (e.metaKey && e.key === "c") {
    e.preventDefault();
    console.log("copy");
  }
};

const handleSelectedText = () => {
  const selectedText = window.getSelection()?.toString();
  console.log(selectedText);
  if (selectedText) {
    window.ipc.postMessage(
      JSON.stringify({ selectedText: { widget_id: "1", text: selectedText } })
    );
  }
};

// Example usage
const App = () => {
  // useEffect(() => {
  //   window.addEventListener("keydown", handleCopyPaste);
  //   window.addEventListener("selectionchange", handleSelectedText);
  //   return () => {
  //     window.removeEventListener("keydown", handleCopyPaste);
  //     window.removeEventListener("selectionchange", handleSelectedText);
  //   };
  // }, []);
  return (
    <>
      <h1>Hello from React</h1>
      <NewWidgetForm />
    </>
  );
};

export default App;
