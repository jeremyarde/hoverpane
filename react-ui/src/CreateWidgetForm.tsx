import { useState } from "react";
import { CreateWidgetRequest, Level, WidgetType } from "./types";

const stack = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={1.5}
    stroke="currentColor"
    className="size-6"
  >
    <rect
      x="3.75"
      y="3.75"
      width="12"
      height="12"
      rx="2.25"
      fill="white"
      opacity="1"
      stroke="black"
    />
    <rect
      x="8"
      y="8"
      width="12"
      height="12"
      rx="2.25"
      fill="black"
      opacity="1"
      stroke="black"
    />
  </svg>
);
const stackFront = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={1.5}
    stroke="currentColor"
    className="size-6"
  >
    <rect
      x="3.75"
      y="3.75"
      width="12"
      height="12"
      rx="2.25"
      fill="black"
      opacity="1"
      stroke="none"
    />
    <rect
      x="8"
      y="8"
      width="12"
      height="12"
      rx="2.25"
      fill="white"
      opacity="1"
      stroke="black"
    />
  </svg>
);

// Add modifier icon
const modifierIcon = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={1.5}
    stroke="currentColor"
    className="size-5"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
    />
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
    />
  </svg>
);

// Add refresh icon
const refreshIcon = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={1.5}
    stroke="currentColor"
    className="size-4"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
    />
  </svg>
);

// Add scrape/extract icon
const scrapeIcon = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={1.5}
    stroke="currentColor"
    className="size-4"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M15.75 15.75l-2.489-2.489m0 0a3.375 3.375 0 10-4.773-4.773 3.375 3.375 0 004.774 4.774zM21 12a9 9 0 11-18 0 9 9 0 0118 0z"
    />
  </svg>
);

// Add help icon
const helpIcon = (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    fill="none"
    viewBox="0 0 24 24"
    strokeWidth={1.5}
    stroke="currentColor"
    className="size-4 text-gray-400"
  >
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z"
    />
  </svg>
);

const DEFAULT_WIDGET_X = 100;
const DEFAULT_WIDGET_Y = 100;
const DEFAULT_WIDGET_WIDTH = 800;
const DEFAULT_WIDGET_HEIGHT = 600;

type ApiError = {
  message: string;
  origin: string;
};

const defaultValues: CreateWidgetRequest = {
  url: "",
  html: "",
  title: "",
  level: Level.Normal,
  transparent: false,
  decorations: false,
  modifiers: [],
  bounds: {
    x: DEFAULT_WIDGET_X,
    y: DEFAULT_WIDGET_Y,
    width: DEFAULT_WIDGET_WIDTH,
    height: DEFAULT_WIDGET_HEIGHT,
  },
};

export default function CreateWidgetForm() {
  const [error, setError] = useState<ApiError | null>(null);
  const [widgetType, setWidgetType] = useState<WidgetType>({
    type: "url",
    content: { url: "" },
  });
  const [selectedLevel, setSelectedLevel] = useState<Level>(Level.Normal);
  const [showModifiers, setShowModifiers] = useState(false);
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [scrapeValue, setScrapeValue] = useState(false);
  const [refreshInterval, setRefreshInterval] = useState(30);
  const [selector, setSelector] = useState("");
  const [bounds, setBounds] = useState({
    x: DEFAULT_WIDGET_X,
    y: DEFAULT_WIDGET_Y,
    width: DEFAULT_WIDGET_WIDTH,
    height: DEFAULT_WIDGET_HEIGHT,
  });

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data: CreateWidgetRequest = {
      url:
        widgetType.type === "url" ? (formData.get("url") as string) ?? "" : "",
      html:
        widgetType.type === "file"
          ? (formData.get("html") as string) ?? ""
          : "",
      title: (formData.get("title") as string) ?? "",
      level: selectedLevel,
      transparent: formData.get("transparent") === "on",
      decorations: false,
      modifiers: [],
      bounds: bounds,
    };

    // Add modifiers if enabled
    if (widgetType.type === "url") {
      if (autoRefresh && refreshInterval >= 5) {
        data.modifiers.push({
          type: "refresh",
          content: {
            modifier_id: "",
            interval_sec: refreshInterval,
          },
        });
      }
      if (scrapeValue && selector.trim()) {
        data.modifiers.push({
          type: "scrape",
          content: {
            modifier_id: "",
            selector: selector.trim(),
          },
        });
      }
    }

    console.log("Form submitted:", data);

    try {
      const res = await fetch(`http://127.0.0.1:3111/widgets`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (res.ok) {
        setError(null);
        const widget = await res.json();
        console.log("Widget created:", widget);

        setWidgetType({ type: "url", content: { url: "" } });
        setSelectedLevel(Level.Normal);
        setShowModifiers(false);
        setAutoRefresh(false);
        setScrapeValue(false);
        setRefreshInterval(30);
        setSelector("");
        setBounds({
          x: DEFAULT_WIDGET_X,
          y: DEFAULT_WIDGET_Y,
          width: DEFAULT_WIDGET_WIDTH,
          height: DEFAULT_WIDGET_HEIGHT,
        });
      } else {
        const error = await res.json();
        setError(error);
      }
    } catch (fetchError) {
      setError({
        message: "Network error or API unreachable",
        origin: "fetch",
      });
      console.error("Fetch error:", fetchError);
    }
  };

  const handleReset = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setError(null);
    setWidgetType({ type: "url", content: { url: "" } });
    setSelectedLevel(Level.Normal);
    setShowModifiers(false);
    setAutoRefresh(false);
    setScrapeValue(false);
    setRefreshInterval(30);
    setSelector("");
    setBounds({
      x: DEFAULT_WIDGET_X,
      y: DEFAULT_WIDGET_Y,
      width: DEFAULT_WIDGET_WIDTH,
      height: DEFAULT_WIDGET_HEIGHT,
    });
  };

  const inputClass = "border rounded p-1 w-full text-sm";
  const labelClass = "block text-xs font-medium mb-0.5";

  return (
    <div className="max-w-md mx-auto">
      <form
        onSubmit={handleSubmit}
        onReset={handleReset}
        className="p-3 space-y-2"
      >
        <h2 className="font-semibold">Create New Widget</h2>
        {/* Widget Type Toggle */}
        <div className="flex border rounded overflow-hidden">
          <button
            type="button"
            className={`flex-1 p-1 text-center text-sm ${
              widgetType.type === "url"
                ? "bg-blue-500 text-white"
                : "bg-gray-200"
            }`}
            onClick={() => setWidgetType({ type: "url", content: { url: "" } })}
          >
            URL
          </button>
          <button
            type="button"
            className={`flex-1 p-1 text-center text-sm ${
              widgetType.type === "file"
                ? "bg-blue-500 text-white"
                : "bg-gray-200"
            }`}
            onClick={() =>
              setWidgetType({ type: "file", content: { html: "" } })
            }
          >
            HTML/JS
          </button>
        </div>

        {/* URL Field */}
        {widgetType.type === "url" && (
          <div>
            <label htmlFor="url" className={labelClass}>
              URL
            </label>
            <input
              type="text"
              id="url"
              name="url"
              defaultValue={defaultValues.url}
              placeholder="https://example.com"
              className={inputClass}
              required
            />
          </div>
        )}

        {/* HTML Field */}
        {widgetType.type === "file" && (
          <div>
            <label htmlFor="html" className={labelClass}>
              HTML Content
            </label>
            <textarea
              id="html"
              name="html"
              defaultValue={defaultValues.html}
              placeholder="<div>Hello World</div>"
              className={`${inputClass} min-h-[80px]`}
              rows={4}
              required
            />
          </div>
        )}

        {/* Title Field */}
        <div>
          <label htmlFor="title" className={labelClass}>
            Title
          </label>
          <input
            type="text"
            id="title"
            name="title"
            defaultValue={defaultValues.title}
            placeholder="Widget Title"
            className={inputClass}
          />
        </div>

        {/* Window Level Field */}
        <div>
          <label className={labelClass}>Window Level</label>
          <div className="flex border rounded overflow-hidden">
            <button
              type="button"
              className={`flex-1 flex items-center justify-center gap-1 p-1.5 text-sm transition-colors ${
                selectedLevel === Level.AlwaysOnBottom
                  ? "bg-blue-500 text-white"
                  : "bg-gray-200 hover:bg-gray-300"
              }`}
              onClick={() => setSelectedLevel(Level.AlwaysOnBottom)}
            >
              {stack}
              Bottom
            </button>
            <button
              type="button"
              className={`flex-1 flex items-center justify-center gap-1 p-1.5 text-sm transition-colors ${
                selectedLevel === Level.Normal
                  ? "bg-blue-500 text-white"
                  : "bg-gray-200 hover:bg-gray-300"
              }`}
              onClick={() => setSelectedLevel(Level.Normal)}
            >
              Normal
            </button>
            <button
              type="button"
              className={`flex-1 flex items-center justify-center gap-1 p-1.5 text-sm transition-colors ${
                selectedLevel === Level.AlwaysOnTop
                  ? "bg-blue-500 text-white"
                  : "bg-gray-200 hover:bg-gray-300"
              }`}
              onClick={() => setSelectedLevel(Level.AlwaysOnTop)}
            >
              <div className="flex items-center gap-1">
                {stackFront}
                Top
              </div>
            </button>
          </div>
        </div>

        {/* Transparent Field */}
        <div className="flex items-center space-x-1.5">
          <input
            type="checkbox"
            id="transparent"
            name="transparent"
            defaultChecked={defaultValues.transparent}
            className="h-3.5 w-3.5 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
          />
          <label htmlFor="transparent" className="text-xs font-medium">
            Transparent Background
          </label>
          <input
            type="checkbox"
            id="decorations"
            name="decorations"
            defaultChecked={defaultValues.decorations}
            className="h-3.5 w-3.5 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
          />
          <label htmlFor="decorations" className="text-xs font-medium">
            Decorations
          </label>
        </div>

        {/* Add Modifiers Button - Only show for URL type */}
        {widgetType.type === "url" && (
          <div className="pt-2">
            <button
              type="button"
              onClick={() => setShowModifiers(!showModifiers)}
              className={`w-full flex items-center justify-center gap-2 p-1.5 text-sm transition-colors border rounded
                ${
                  showModifiers
                    ? "bg-blue-500 text-white border-blue-600"
                    : "bg-gray-100 text-gray-700 border-gray-300 hover:bg-gray-200"
                }`}
            >
              {modifierIcon}
              {showModifiers ? "Hide Modifiers" : "Add Modifiers"}
            </button>
          </div>
        )}

        {/* Modifiers Panel */}
        {showModifiers && widgetType.type === "url" && (
          <div className="border rounded p-3 space-y-3 bg-gray-50">
            {/* Auto Refresh Section */}
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="text-sm font-medium flex items-center gap-1.5">
                    {refreshIcon}
                    Auto Refresh
                  </div>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={autoRefresh}
                    onChange={(e) => setAutoRefresh(e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-blue-500"></div>
                </label>
              </div>
              {autoRefresh && (
                <div className="pl-6">
                  <label className="block text-xs font-medium mb-1">
                    Refresh Interval (seconds)
                  </label>
                  <input
                    type="number"
                    min="5"
                    max="3600"
                    value={refreshInterval}
                    onChange={(e) => setRefreshInterval(Number(e.target.value))}
                    className="w-full px-2 py-1 text-sm border rounded focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                </div>
              )}
            </div>

            {/* Scrape Value Section */}
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <div className="text-sm font-medium flex items-center gap-1.5">
                    {scrapeIcon}
                    Scrape Value
                    <div className="group relative">
                      {helpIcon}
                      <div className="absolute left-0 bottom-full mb-2 hidden group-hover:block w-64 p-2 bg-gray-800 text-white text-xs rounded shadow-lg">
                        Use a CSS selector to extract specific content from the
                        webpage. Examples:
                        <ul className="mt-1 ml-2 list-disc">
                          <li>#price-value</li>
                          <li>.stock-price</li>
                          <li>span.temperature</li>
                        </ul>
                      </div>
                    </div>
                  </div>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={scrapeValue}
                    onChange={(e) => setScrapeValue(e.target.checked)}
                    className="sr-only peer"
                  />
                  <div className="w-9 h-5 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:bg-blue-500"></div>
                </label>
              </div>
              {scrapeValue && (
                <div className="pl-6">
                  <label className="block text-xs font-medium mb-1">
                    CSS Selector
                  </label>
                  <input
                    type="text"
                    value={selector}
                    onChange={(e) => setSelector(e.target.value)}
                    placeholder="#price, .value, etc."
                    className={`w-full px-2 py-1 text-sm border rounded focus:outline-none focus:ring-1 focus:ring-blue-500 ${
                      scrapeValue && !selector.trim() ? "border-red-300" : ""
                    }`}
                  />
                  <p className="text-xs text-gray-500 mt-1">
                    Enter a CSS selector to extract specific content from the
                    page
                  </p>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Size and Location Fields */}
        <div className="space-y-2">
          <label className={labelClass}>Size and Location</label>
          <div className="grid grid-cols-2 gap-2">
            <div>
              <label className="block text-xs text-gray-500 mb-0.5">
                X Position
              </label>
              <input
                type="number"
                value={bounds.x}
                onChange={(e) =>
                  setBounds({ ...bounds, x: Number(e.target.value) })
                }
                className={inputClass}
                min="0"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-0.5">
                Y Position
              </label>
              <input
                type="number"
                value={bounds.y}
                onChange={(e) =>
                  setBounds({ ...bounds, y: Number(e.target.value) })
                }
                className={inputClass}
                min="0"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-0.5">
                Width
              </label>
              <input
                type="number"
                value={bounds.width}
                onChange={(e) =>
                  setBounds({ ...bounds, width: Number(e.target.value) })
                }
                className={inputClass}
                min="100"
              />
            </div>
            <div>
              <label className="block text-xs text-gray-500 mb-0.5">
                Height
              </label>
              <input
                type="number"
                value={bounds.height}
                onChange={(e) =>
                  setBounds({ ...bounds, height: Number(e.target.value) })
                }
                className={inputClass}
                min="100"
              />
            </div>
          </div>
        </div>

        {/* Form Actions */}
        <div className="flex justify-end space-x-2">
          <button
            type="reset"
            className="px-3 py-1 border border-gray-300 rounded-md text-xs font-medium hover:bg-gray-50"
          >
            Reset
          </button>
          <button
            type="submit"
            className="px-3 py-1 border border-transparent rounded-md shadow-sm text-xs font-medium text-white bg-blue-600 hover:bg-blue-700"
          >
            Create Widget
          </button>
        </div>
      </form>
      {error && (
        <div className="mt-2 p-2 bg-red-100 border border-red-400 text-red-700 rounded text-xs">
          <p className="font-medium">Error:</p>
          <p>{error.message}</p>
        </div>
      )}
    </div>
  );
}
