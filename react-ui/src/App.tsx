import {
  createContext,
  useContext,
  useState,
  useEffect,
  ReactNode,
} from "react";
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

interface NewViewConfig {
  // id: string;
  url: string;
  selector: string;
  refreshInterval?: number;
  // position?: {
  //   x: number;
  //   y: number;
  // };
}

// Add validation methods
interface NewViewConfigValidation {
  isValid(config: NewViewConfig): boolean;
  getErrors(config: NewViewConfig): string[];
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
// Implementation
const viewConfigValidation: NewViewConfigValidation = {
  isValid(config: NewViewConfig): boolean {
    return !this.getErrors(config).length;
  },

  getErrors(config: NewViewConfig): string[] {
    const errors: string[] = [];

    // if (!config.id.trim()) {
    //   errors.push("ID is required");
    // }

    if (!config.url.trim()) {
      errors.push("URL is required");
    } else if (!isValidUrl(config.url)) {
      errors.push("Invalid URL format");
    }

    if (!config.selector.trim()) {
      errors.push("Selector is required");
    }

    if (config.refreshInterval && config.refreshInterval < 1000) {
      errors.push("Refresh interval must be at least 1000ms");
    }

    return errors;
  },
};

// Usage in form
const NewViewForm: React.FC = () => {
  const [config, setConfig] = useState<NewViewConfig>({
    url: "",
    selector: "",
    refreshInterval: 10,
  });
  const [errors, setErrors] = useState<string[]>([]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const validationErrors = viewConfigValidation.getErrors(config);
    if (validationErrors.length) {
      setErrors(validationErrors);
      return;
    }
    window.ipc.postMessage(JSON.stringify({ createView: config }));
  };

  return (
    <form onSubmit={handleSubmit}>
      {errors.map((error) => (
        <div key={error} className="error">
          {error}
        </div>
      ))}
      <input
        required
        placeholder="URL"
        value={config.url}
        onChange={(e) => setConfig({ ...config, url: e.target.value })}
      />
      <input
        required
        placeholder="Selector"
        value={config.selector}
        onChange={(e) => setConfig({ ...config, selector: e.target.value })}
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
      <NewViewForm />
    </>
  );
};

export default App;
