try {
  const element = document.querySelector("$selector");
  if (!element) {
    window.ipc.postMessage(
      JSON.stringify({
        error: "Element not found",
        value: null,
        id: "$id",
      })
    );
  }

  window.ipc.postMessage(
    JSON.stringify({
      error: null,
      value: element,
      id: "$id",
    })
  );
} catch (e) {
  window.ipc.postMessage(
    JSON.stringify({
      error: e.message,
      value: null,
      id: "$id",
    })
  );
}
