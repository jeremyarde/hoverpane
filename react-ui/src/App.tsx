/* eslint-disable @typescript-eslint/no-unused-vars */

import { createContext, useContext, useState, useEffect } from "react";
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
  const [formData, setFormData] = useState({
    title: "",
    "widget-type": "source",
    url: "",
    selector: "",
    "refresh-interval": "1000",
  });

  // Update form data when paste occurs
  useEffect(() => {
    window.onRustMessage = (rawMessage: string) => {
      try {
        const message = JSON.parse(rawMessage);
        if (message.paste) {
          // Update the focused input
          const activeElement = document.activeElement as HTMLInputElement;
          if (activeElement && activeElement.name) {
            setFormData((prev) => ({
              ...prev,
              [activeElement.name]: message.paste,
            }));
          }
        }
      } catch (e) {
        console.error("Failed to parse message:", e);
      }
    };
  }, []);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const message = {
      createwidget: {
        id: "fakeid",
        title: formData.title,
        refresh_interval: parseInt(formData["refresh-interval"]),
        widget_type: {
          source: {
            url: formData.url,
            element_selectors: [formData.selector],
            refresh_interval: parseInt(formData["refresh-interval"]),
          },
        },
      },
    };

    console.log("Creating a widget: ", message);
    window.ipc.postMessage(JSON.stringify(message));
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  return (
    <form onSubmit={handleSubmit} className="newview-form">
      <input
        className="newview-form-input"
        placeholder="Title"
        name="title"
        value={formData["title"]}
        onChange={handleChange}
      />
      <select
        className="newview-form-input"
        required
        name="widget-type"
        value={formData["widget-type"]}
        onChange={handleChange}
      >
        <option value="source">Source</option>
        <option value="display">Display</option>
        <option value="tracker">Tracker</option>
        <option value="controls">Controls</option>
      </select>
      <input
        className="newview-form-input"
        placeholder="URL"
        name="url"
        value={formData["url"]}
        onChange={handleChange}
      />
      <input
        className="newview-form-input"
        placeholder="Selector"
        name="selector"
        value={formData["selector"]}
        onChange={handleChange}
      />
      <input
        className="newview-form-input"
        placeholder="Refresh Interval"
        name="refresh-interval"
        type="number"
        value={formData["refresh-interval"]}
        onChange={handleChange}
      />
      <button type="submit">Create View</button>
    </form>
  );
};

const handleCopyPaste = (e: KeyboardEvent) => {
  // if (e.metaKey && e.key === "v") {
  //   e.preventDefault();
  //   window.ipc.postMessage(
  //     JSON.stringify({
  //       pastetext: {
  //         widget_id: window.WIDGET_ID ? window.WIDGET_ID : "not-set",
  //       },
  //     })
  //   );
  // }
  // if (e.metaKey && e.key === "c") {
  //   e.preventDefault();
  //   window.ipc.postMessage(
  //     JSON.stringify({
  //       copytext: {
  //         widget_id: window.WIDGET_ID ? window.WIDGET_ID : "not-set",
  //         text: window.getSelection()?.toString(),
  //       },
  //     })
  //   );
  // }
};

const handleSelectedText = () => {
  const selectedText = window.getSelection()?.toString();
  console.log(selectedText);
  if (selectedText) {
    window.ipc.postMessage(
      JSON.stringify({
        selectedtext: {
          widget_id: window.WIDGET_ID ? window.WIDGET_ID : "not-set",
          text: selectedText,
        },
      })
    );
  }
};

// Example usage
const App = () => {
  const [messages, setMessages] = useState<Message[]>([]);

  useEffect(() => {
    // Set up the Rust message handler
    window.onRustMessage = (rawMessage: string) => {
      try {
        console.log("Received message from Rust:", rawMessage);
        const message = JSON.parse(rawMessage);

        setMessages((prev) => [...prev, message]);

        // if (message.paste) {
        //   console.log("Pasting text:", message.text);
        //   const pasteEvent = new ClipboardEvent("paste", {
        //     clipboardData: new DataTransfer(),
        //     bubbles: true,
        //     cancelable: true,
        //   });
        //   // Set clipboard data
        //   pasteEvent.clipboardData?.setData("text/plain", message.text);
        //   // Dispatch the event
        //   document.activeElement?.dispatchEvent(pasteEvent);
        // }

        if (message.paste) {
          console.log("Pasting text:", message.paste);
          const input = document.activeElement as HTMLInputElement;
          if (input) {
            input.setAttribute("value", message.paste);
            input.dispatchEvent(new Event("input", { bubbles: true }));
          }
        }

        // Handle different message types
        // switch (message.data_key) {
        //   case "widget_update":
        //     // Handle widget updates
        //     break;
        //   case "error":
        //     console.error("Error from Rust:", message.message);
        //     break;
        //   default:
        //     console.log("Unknown message type:", message);
        // }
      } catch (e) {
        console.error("Failed to parse message from Rust:", e);
      }
    };

    return () => {
      window.onRustMessage = () => {};
    };
  }, []);

  useEffect(() => {
    window.addEventListener("keydown", handleCopyPaste);
    window.addEventListener("selectionchange", handleSelectedText);
    return () => {
      window.removeEventListener("keydown", handleCopyPaste);
      window.removeEventListener("selectionchange", handleSelectedText);
    };
  }, []);
  return (
    <>
      <h1>Hello from React</h1>
      <NewWidgetForm />
    </>
  );
};

export default App;
