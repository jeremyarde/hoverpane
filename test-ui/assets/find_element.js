setInterval(() => {
  const element = document.querySelector("$pattern");
  if (element) {
    window.ipc.postMessage(
      JSON.stringify({ value: element.textContent, view_id: $view_id })
    );
  }
}, parseInt("$interval"));
