(function () {
  const interval = setInterval(() => {
    const element = document.querySelector("$pattern");
    console.log("Scraped element: ", element);
    if (element) {
      window.ipc.postMessage(
        JSON.stringify({ value: element.textContent, view_id: "$view_id" })
      );
    }
  }, parseInt("$interval"));
})();
