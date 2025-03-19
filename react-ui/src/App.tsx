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

// Example usage
const App = () => {
  const [views, setViews] = useState<ReactNode[]>([]);
  const [filter, setFilter] = useState<string>("");
  return (
    <MessageProvider>
      <input type="text" onChange={(e) => setFilter(e.target.value)} />
      <button
        onClick={() => setViews([...views, <DataView filter={filter} />])}
      >
        Add DataView
      </button>
      <div>
        {views.map((view, index) => (
          <div key={index}>{view}</div>
        ))}
      </div>
    </MessageProvider>
  );
};

export default App;
