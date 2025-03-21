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

// Provider component to manage messages
export const MessageProvider = ({ children }: { children: ReactNode }) => {
  const [messages, setMessages] = useState<Message[]>([]);

  const addMessage = (message: Message) => {
    setMessages((prev) => [...prev, message]);
  };

  // Set up the Rust message handler
  useEffect(() => {
    window.onRustMessage = (message: string) => {
      addMessage(JSON.parse(message));
    };

    return () => {
      window.onRustMessage = () => {};
    };
  }, []);

  return (
    <MessageContext.Provider value={{ messages, addMessage }}>
      {children}
    </MessageContext.Provider>
  );
};

// Custom hook to use the message context
export const useMessages = () => {
  return useContext(MessageContext);
};

const DataView = ({ filter }: { filter: string }) => {
  const { messages } = useMessages();
  const filteredMessages = messages.filter((message) => {
    console.log("filtering messages", message);
    return JSON.parse(message.message).key.toString().includes(filter);
  });
  return (
    <div>
      DataView
      <div>{filter}</div>
      <div>{JSON.stringify(filteredMessages)}</div>
    </div>
  );
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
    // id: "",
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
    window.ipc.postMessage("createView", config);
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

// Example usage
const App = () => {
  // const [views, setViews] = useState<ReactNode[]>([]);
  // const [filter, setFilter] = useState<string>("");
  // const [values, setValues] = useState<Record[]>([]);
  return (
    <>
      <NewViewForm />
    </>
  );
};

export default App;
