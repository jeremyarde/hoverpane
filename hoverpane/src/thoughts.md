
Desktop app that runs hidden webviews, and scrapes certain values from the html using selectors (or other methods?) and displays them in a UI.

Current:
- One window, creates webviews configured by the user, scrapes values from them, sends them to the UI.
- We want to be able to have multiple windows, each with their own webviews, and displaying the values in the UI.

Issues:
- sending messages from webview to UI is not nice.
- want the user to be able to configure how to display the data.
- format of the data needs to be flexible, but also easy to understand for users to configure the ui to display the data.

Questions:
- should I split out the desktop "scraper" into a separate process?
- how store the data from the webviews that gets scraped, and make it available to the UI?
- how to handle the configuration of the UI?
  - UI being configurable (bring your own html/js, react editor, other???)






