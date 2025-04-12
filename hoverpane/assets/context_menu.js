(function () {
  if (!document.body) return; // Ensure body exists

  const menu = document.createElement("div");
  menu.style.position = "absolute";
  menu.style.background = "#fff";
  menu.style.border = "1px solid #ccc";
  menu.style.padding = "10px";
  menu.style.boxShadow = "2px 2px 10px rgba(0,0,0,0.2)";
  menu.style.display = "none";
  menu.style.zIndex = "1000";

  menu.innerHTML = `
      <div style="padding: 5px; cursor: pointer;">Option 1</div>
      <div style="padding: 5px; cursor: pointer;">Option 2</div>
      <div style="padding: 5px; cursor: pointer;">Option 3</div>
  `;

  document.body.appendChild(menu);

  document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
    menu.style.left = `${event.pageX}px`;
    menu.style.top = `${event.pageY}px`;
    menu.style.display = "block";
  });

  document.addEventListener("click", () => {
    menu.style.display = "none";
  });
})();
