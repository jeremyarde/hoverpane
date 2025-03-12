document.addEventListener("DOMContentLoaded", async () => {
  let list = document.getElementById("tracked-list");
  let storage = await chrome.storage.local.get("tracked");
  let tracked = storage.tracked || [];

  tracked.forEach(({ selector, text }) => {
    let li = document.createElement("li");
    li.textContent = `${text} (${selector})`;
    list.appendChild(li);
  });
});

chrome.runtime.onMessage.addListener(async (message) => {
  let tracked = (await chrome.storage.local.get("tracked")).tracked || [];
  tracked.push(message);
  await chrome.storage.local.set({ tracked });

  chrome.action.setBadgeText({ text: tracked.length.toString() });
});
