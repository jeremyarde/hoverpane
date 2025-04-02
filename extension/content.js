// Create a highlight box element
const highlightBox = document.createElement("div");
highlightBox.style.position = "fixed";
highlightBox.style.border = "2px solid #007bff";
highlightBox.style.backgroundColor = "rgba(0, 123, 255, 0.1)";
highlightBox.style.pointerEvents = "none"; // Make sure the box doesn't interfere with clicks
highlightBox.style.zIndex = "10000";
highlightBox.style.display = "none";
document.body.appendChild(highlightBox);

// Track if highlighting is active and current element
let isHighlightingEnabled = false;
let currentHoveredElement = null;

// Helper function to generate a unique selector
function generateUniqueSelector(el) {
  if (el.id) {
    return `#${CSS.escape(el.id)}`;
  }

  let path = [];
  while (el && el.nodeType === Node.ELEMENT_NODE) {
    let selector = el.tagName.toLowerCase();

    if (el.className) {
      const classes = Array.from(el.classList)
        .map((c) => CSS.escape(c))
        .join(".");
      if (classes) {
        selector += "." + classes;
      }
    }

    // Add nth-child for more specificity
    let nth = 1;
    let sibling = el.previousElementSibling;
    while (sibling) {
      nth++;
      sibling = sibling.previousElementSibling;
    }
    selector += `:nth-child(${nth})`;

    path.unshift(selector);
    el = el.parentElement;
  }

  return path.join(" > ");
}

// Listen for messages from the background script
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log("Content script received message:", message);

  // Handle ping message
  if (message.action === "ping") {
    console.log("Responding to ping");
    sendResponse({ status: "ok" });
    return false; // No async response needed
  }

  if (message.action === "toggleHighlight") {
    console.log("Toggling highlight:", message.enabled);
    isHighlightingEnabled = message.enabled;
    highlightBox.style.display = "none";
    if (isHighlightingEnabled) {
      document.body.style.cursor = "crosshair";
    } else {
      document.body.style.cursor = "";
      currentHoveredElement = null;
    }
    sendResponse({ status: "ok" });
    return false; // No async response needed
  }

  if (message.action === "getElementInfo") {
    console.log(
      "Getting element info, current element:",
      currentHoveredElement
    );
    // if (!currentHighlightedElement) {
    //   console.warn("No element currently highlighted");
    //   sendResponse({
    //     status: "error",
    //     error: "No element currently highlighted",
    //   });
    //   return false;
    // }

    try {
      const selector = generateUniqueSelector(currentHoveredElement);
      const elementInfo = {
        selector,
        text:
          currentHoveredElement.innerText || currentHoveredElement.value || "",
        url: window.location.href,
        timestamp: Date.now(),
      };
      console.log("Sending element info:", elementInfo);
      chrome.runtime.sendMessage(elementInfo, (response) => {
        console.log("Got response from background:", response);
        sendResponse({ status: "ok", ...response });
      });
      return true; // Will send response asynchronously
    } catch (error) {
      console.error("Error processing element:", error);
      chrome.runtime.sendMessage(
        {
          error: "Failed to process element: " + error.message,
        },
        (response) => {
          sendResponse({ status: "error", error: error.message, ...response });
        }
      );
      return true; // Will send response asynchronously
    }
  }

  console.log("Unhandled message action:", message.action);
  sendResponse({ status: "ok" });
  return false; // No async response needed
});

// Handle mouse movement
document.addEventListener("mousemove", (e) => {
  if (!isHighlightingEnabled) return;

  const target = e.target;
  if (target === highlightBox) return;

  currentHoveredElement = target;
  const rect = target.getBoundingClientRect();

  // Update highlight box position and size
  highlightBox.style.display = "block";
  highlightBox.style.top = `${rect.top + window.scrollY}px`;
  highlightBox.style.left = `${rect.left + window.scrollX}px`;
  highlightBox.style.width = `${rect.width}px`;
  highlightBox.style.height = `${rect.height}px`;

  // Add tooltip with element info
  const tagName = target.tagName.toLowerCase();
  const className = target.className
    ? `.${target.className.split(" ").join(".")}`
    : "";
  const id = target.id ? `#${target.id}` : "";
  const text = target.innerText || target.value || "";
  highlightBox.title = `${tagName}${id}${className}\n${text.slice(0, 50)}${
    text.length > 50 ? "..." : ""
  }`;
});

// Handle mouse leaving elements
document.addEventListener("mouseout", (e) => {
  if (!isHighlightingEnabled) return;
  if (!e.relatedTarget) {
    highlightBox.style.display = "none";
    currentHoveredElement = null;
  }
});

// Handle scrolling
document.addEventListener("scroll", () => {
  if (!isHighlightingEnabled || !currentHoveredElement) return;

  const rect = currentHoveredElement.getBoundingClientRect();
  highlightBox.style.top = `${rect.top + window.scrollY}px`;
  highlightBox.style.left = `${rect.left + window.scrollX}px`;
});
