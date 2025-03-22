// Helper function for clipboard operations
async function handleClipboard(operation) {
  try {
    if (!navigator.clipboard) {
      throw new Error("Clipboard API not available");
    }

    if (operation === "paste") {
      const text = await navigator.clipboard.readText();
      console.log("Pasted text:", text);
      return text;
    } else if (operation === "copy") {
      const selection = window.getSelection()?.toString();
      if (selection) {
        await navigator.clipboard.writeText(selection);
        console.log("Copied text:", selection);
      }
    }
  } catch (e) {
    console.error("Clipboard operation failed:", e);
    // Notify Rust about the error
    // window.ipc?.postMessage("clipboardError", e.message);
  }
}

document.addEventListener("keydown", (event) => {
  // if ((event.ctrlKey || event.metaKey) && event.key === "c") {
  //     window.ipc.postMessage(
  //       JSON.stringify({
  //         copy: {
  //           widget_id: "$widget_id",
  //           text: selection.toString(),
  //         },
  //       })
  //     );
  //   }

  if ((event.ctrlKey || event.metaKey) && event.key === "v") {
    console.log("Keyboard Paste detected!");
    handleClipboard("paste");
  }
});

// Add event listeners
document.addEventListener("paste", async (e) => {
  e.preventDefault();
  const text = await handleClipboard("paste");
  if (text) {
    // Handle the pasted text
    const target = e.target;
    if (
      target instanceof HTMLInputElement ||
      target instanceof HTMLTextAreaElement
    ) {
      const start = target.selectionStart || 0;
      const end = target.selectionEnd || 0;
      target.value =
        target.value.slice(0, start) + text + target.value.slice(end);
      target.selectionStart = target.selectionEnd = start + text.length;
    }
  }
});

document.addEventListener("copy", (e) => {
  handleClipboard("copy");
});
