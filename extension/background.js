// Track highlighting state
let isHighlightingEnabled = false;

// Initialize WebSocket connection and context menus when extension is installed
chrome.runtime.onInstalled.addListener(() => {
  setupContextMenus();
  setupWebSocket();
});

// Listen for tab updates to ensure content script is loaded
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete" && tab.url?.startsWith("http")) {
    // Inject content script if needed
    chrome.scripting
      .executeScript({
        target: { tabId: tabId },
        files: ["content.js"],
      })
      .catch((error) => {
        console.warn("Content script injection failed:", error);
      });
  }
});

// Create context menus
function setupContextMenus() {
  chrome.contextMenus.create({
    id: "trackElement",
    title: "Track this element",
    contexts: ["all"],
  });

  chrome.contextMenus.create({
    id: "toggleHighlight",
    title: "Enable element highlighting",
    contexts: ["all"],
  });
}

// WebSocket connection management
let socket = null;
let reconnectAttempts = 0;
let maxReconnectAttempts = 5;
let reconnectTimeout = 1000; // Start with 1 second
const maxReconnectTimeout = 30000; // Max 30 seconds
let pendingMessages = new Map(); // Store messages waiting for ack
let messageTimeout = 5000; // 5 seconds timeout for ack
let consecutiveFailedMessages = 0;
const maxFailedMessages = 3;

// Check if a tab is eligible for content script injection
async function isValidTab(tab) {
  if (!tab) return false;
  return tab.url && tab.url.startsWith("http");
}

// Check if content script is available and responsive
function isContentScriptAvailable(tabId) {
  return new Promise((resolve) => {
    chrome.tabs.sendMessage(tabId, { action: "ping" }, (response) => {
      if (chrome.runtime.lastError) {
        // If content script is not loaded, try to inject it
        if (
          chrome.runtime.lastError.message.includes(
            "Receiving end does not exist"
          )
        ) {
          chrome.scripting
            .executeScript({
              target: { tabId: tabId },
              files: ["content.js"],
            })
            .then(() => {
              // Retry the ping after injection
              setTimeout(() => {
                chrome.tabs.sendMessage(
                  tabId,
                  { action: "ping" },
                  (pingResponse) => {
                    resolve(pingResponse && pingResponse.status === "ok");
                  }
                );
              }, 100);
            })
            .catch((error) => {
              console.warn("Content script injection failed:", error);
              resolve(false);
            });
          return;
        }
        resolve(false);
        return;
      }
      resolve(response && response.status === "ok");
    });
  });
}

// Send message to content script with retry
async function sendMessageToContentScript(tabId, message) {
  console.log("Attempting to send message to content script:", {
    tabId,
    message,
  });

  try {
    // Get current tab to verify it's still valid
    const tab = await chrome.tabs.get(tabId);
    if (!tab) {
      console.warn("Tab not found:", tabId);
      return null;
    }

    // Verify tab is valid for content script
    if (!tab.url?.startsWith("http")) {
      console.warn("Invalid tab URL for content script:", tab.url);
      return null;
    }

    // First ensure content script is loaded
    console.log("Checking content script availability...");
    const isAvailable = await isContentScriptAvailable(tabId);
    console.log("Content script available:", isAvailable);

    if (!isAvailable) {
      // Try injecting the content script directly
      console.log("Attempting to inject content script...");
      try {
        await chrome.scripting.executeScript({
          target: { tabId },
          files: ["content.js"],
        });
        // Wait a bit for the script to initialize
        await new Promise((resolve) => setTimeout(resolve, 100));
      } catch (error) {
        console.error("Failed to inject content script:", error);
        return null;
      }
    }

    // Now try to send the message
    return new Promise((resolve) => {
      console.log("Sending message to tab:", tabId);
      chrome.tabs.sendMessage(tabId, message, (response) => {
        if (chrome.runtime.lastError) {
          console.warn("Send message error:", chrome.runtime.lastError.message);
          resolve(null);
          return;
        }
        console.log("Received response from content script:", response);
        resolve(response);
      });
    });
  } catch (error) {
    console.error("Error in sendMessageToContentScript:", error);
    return null;
  }
}

function setupWebSocket() {
  if (socket && socket.readyState === WebSocket.CONNECTING) {
    return; // Already trying to connect
  }

  socket = new WebSocket("ws://127.0.0.1:8080");

  socket.onopen = () => {
    console.log("Connected to WebSocket server");
    reconnectAttempts = 0;
    reconnectTimeout = 1000;
    // Resend any pending messages
    for (const [id, message] of pendingMessages) {
      sendMessageWithRetry(message, id);
    }
  };

  socket.onclose = () => {
    console.warn("WebSocket disconnected");
    handleReconnect();
  };

  socket.onerror = (error) => {
    console.error("WebSocket Error:", error);
  };

  socket.onmessage = (event) => {
    try {
      const response = JSON.parse(event.data);
      if (response.ack && pendingMessages.has(response.ack)) {
        console.log(`Received ack for message ${response.ack}`);
        pendingMessages.delete(response.ack);
      }
    } catch (error) {
      console.error("Error processing WebSocket message:", error);
    }
  };
}

function handleReconnect() {
  if (reconnectAttempts >= maxReconnectAttempts) {
    console.error("Max reconnection attempts reached");
    return;
  }

  reconnectAttempts++;
  const delay = Math.min(
    reconnectTimeout * reconnectAttempts,
    maxReconnectTimeout
  );
  console.log(
    `Attempting to reconnect in ${delay}ms (attempt ${reconnectAttempts})`
  );
  setTimeout(setupWebSocket, delay);
}

function generateMessageId() {
  return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
}

function sendMessageWithRetry(message, messageId = null) {
  if (!messageId) {
    messageId = generateMessageId();
  }

  if (!socket || socket.readyState !== WebSocket.OPEN) {
    pendingMessages.set(messageId, message);
    if (socket?.readyState !== WebSocket.CONNECTING) {
      setupWebSocket();
    }
    return;
  }

  const messageWithId = { ...message, id: messageId };
  socket.send(JSON.stringify(messageWithId));

  // Set timeout for acknowledgment
  pendingMessages.set(messageId, message);
  setTimeout(() => {
    if (pendingMessages.has(messageId)) {
      console.warn(`No ack received for message ${messageId}, will retry`);
      pendingMessages.delete(messageId);
      sendMessageWithRetry(message);
    }
  }, messageTimeout);
}

// Handle context menu clicks
chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (!tab || !tab.id) return;

  // Check if we can communicate with the content script
  isValidTab(tab).then(async (isValid) => {
    if (!isValid) {
      console.warn("Cannot interact with this page type");
      return;
    }

    if (info.menuItemId === "toggleHighlight") {
      isHighlightingEnabled = !isHighlightingEnabled;

      // Update the context menu item title
      chrome.contextMenus.update("toggleHighlight", {
        title: `${
          isHighlightingEnabled ? "Disable" : "Enable"
        } element highlighting`,
      });

      // Send message to content script
      await sendMessageToContentScript(tab.id, {
        action: "toggleHighlight",
        enabled: isHighlightingEnabled,
      });
    }

    if (info.menuItemId === "trackElement") {
      await sendMessageToContentScript(tab.id, {
        action: "getElementInfo",
      });
    }
  });
});

// Handle messages from the content script
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log("Received message from content script:", message);

  if (message.error) {
    console.error(message.error);
    sendResponse({ status: "error" });
    return false; // No async response needed
  }

  // Send the message through WebSocket with retry mechanism
  sendMessageWithRetry(message);
  sendResponse({ status: "ok" });
  return false; // No async response needed
});

// Handle extension icon clicks
chrome.action.onClicked.addListener(async (tab) => {
  if (!tab || !tab.id) return;

  // Check if we can communicate with the content script
  const isValid = await isValidTab(tab);
  if (!isValid) {
    console.warn("Cannot interact with this page type");
    return;
  }

  // Toggle highlighting
  isHighlightingEnabled = !isHighlightingEnabled;

  // Update the icon title
  chrome.action.setTitle({
    title: `Click to ${
      isHighlightingEnabled ? "disable" : "enable"
    } element highlighting`,
  });

  // Update context menu if it exists
  try {
    chrome.contextMenus.update("toggleHighlight", {
      title: `${
        isHighlightingEnabled ? "Disable" : "Enable"
      } element highlighting`,
    });
  } catch (error) {
    console.log("Context menu not available");
  }

  // Send message to content script
  await sendMessageToContentScript(tab.id, {
    action: "toggleHighlight",
    enabled: isHighlightingEnabled,
  });
});
