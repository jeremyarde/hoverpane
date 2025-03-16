setInterval(() => {
  const element = document.querySelector("$pattern");
  if (element) {
    window.ipc.postMessage(
      JSON.stringify({ extractresult: element.textContent })
    );
  }
}, 3000);
