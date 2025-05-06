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

// Add step icons
const urlIcon = (
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
      d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244"
    />
  </svg>
);

const appearanceIcon = (
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
      d="M9.53 16.122a3 3 0 00-5.78 1.128 2.25 2.25 0 01-2.4 2.245 4.5 4.5 0 008.4-2.245c0-.399-.078-.78-.22-1.128zm0 0a15.998 15.998 0 003.388-1.62m-5.043-.025a15.994 15.994 0 011.622-3.395m3.42 3.42a15.995 15.995 0 004.764-4.648l3.876-5.814a1.151 1.151 0 00-1.597-1.597L14.146 6.32a15.996 15.996 0 00-4.649 4.763m3.42 3.42a6.776 6.776 0 00-3.42-3.42"
    />
  </svg>
);

const settingsIcon = (
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
      d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
    />
    <path
      strokeLinecap="round"
      strokeLinejoin="round"
      d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
    />
  </svg>
);

const positionIcon = (
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
      d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"
    />
  </svg>
);

const steps = [
  { id: 1, title: "Basic Info", icon: urlIcon },
  { id: 2, title: "Appearance", icon: appearanceIcon },
  { id: 3, title: "Advanced", icon: settingsIcon },
  { id: 4, title: "Position", icon: positionIcon },
];

const DEFAULT_WIDGET_X = 100;
const DEFAULT_WIDGET_Y = 100;
const DEFAULT_WIDGET_WIDTH = 800;
const DEFAULT_WIDGET_HEIGHT = 600;

type ApiError = {
  message: string;
  origin: string;
};

export default function SimpleCreateWidgetForm() {
  const [error, setError] = useState<ApiError | null>(null);
  const [currentStep, setCurrentStep] = useState(1);
  const [validationErrors, setValidationErrors] = useState<
    Record<string, string>
  >({});
  const [widgetType, setWidgetType] = useState<WidgetType>({
    type: "url",
    content: { url: "" },
  });
  const [selectedLevel, setSelectedLevel] = useState<Level>(Level.Normal);
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
  const [formData, setFormData] = useState({
    url: "",
    html: "",
    title: "",
    transparent: false,
  });

  const validateStep = (step: number): boolean => {
    const errors: Record<string, string> = {};

    switch (step) {
      case 1:
        if (widgetType.type === "url" && !formData.url) {
          errors.url = "URL is required";
        } else if (widgetType.type === "file" && !formData.html) {
          errors.html = "HTML content is required";
        }
        break;
      case 2:
        // No validation needed for appearance
        break;
      case 3:
        if (autoRefresh && refreshInterval < 5) {
          errors.refreshInterval =
            "Refresh interval must be at least 5 seconds";
        }
        if (scrapeValue && !selector.trim()) {
          errors.selector = "CSS selector is required when scraping is enabled";
        }
        break;
      case 4:
        if (bounds.width < 100) {
          errors.width = "Width must be at least 100px";
        }
        if (bounds.height < 100) {
          errors.height = "Height must be at least 100px";
        }
        break;
    }

    setValidationErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(currentStep)) {
      setCurrentStep((prev) => Math.min(prev + 1, 4));
    }
  };

  const handleBack = () => {
    setCurrentStep((prev) => Math.max(prev - 1, 1));
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!validateStep(currentStep)) {
      return;
    }

    const data: CreateWidgetRequest = {
      url: widgetType.type === "url" ? formData.url : "",
      html: widgetType.type === "file" ? formData.html : "",
      title: formData.title,
      level: selectedLevel,
      transparent: formData.transparent,
      decorations: false,
      modifiers: [],
      bounds: bounds,
    };

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
        resetForm();
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

  const resetForm = () => {
    setCurrentStep(1);
    setValidationErrors({});
    setWidgetType({ type: "url", content: { url: "" } });
    setSelectedLevel(Level.Normal);
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
    setFormData({
      url: "",
      html: "",
      title: "",
      transparent: false,
    });
  };

  const inputClass = "border rounded p-1 w-full text-sm";
  const labelClass = "block text-xs font-medium mb-0.5";

  const renderStepIndicator = () => (
    <div className="mb-8">
      <div className="flex justify-between mb-2">
        {steps.map((step) => (
          <div
            key={step.id}
            className={`flex flex-col items-center ${
              step.id <= currentStep ? "text-blue-600" : "text-gray-400"
            }`}
          >
            <div
              className={`w-8 h-8 rounded-full flex items-center justify-center mb-1 ${
                step.id <= currentStep ? "bg-blue-100" : "bg-gray-100"
              }`}
            >
              {step.icon}
            </div>
            <span className="text-xs">{step.title}</span>
          </div>
        ))}
      </div>
      <div className="flex justify-between">
        {steps.map((step) => (
          <div
            key={step.id}
            className={`flex-1 h-1 mx-1 rounded ${
              step.id <= currentStep ? "bg-blue-500" : "bg-gray-200"
            }`}
          />
        ))}
      </div>
    </div>
  );

  const renderStepContent = () => {
    switch (currentStep) {
      case 1:
        return (
          <div className="space-y-4">
            <h3 className="text-sm font-medium mb-4">Basic Information</h3>
            <div className="flex border rounded overflow-hidden mb-4">
              <button
                type="button"
                className={`flex-1 p-2 text-center text-sm ${
                  widgetType.type === "url"
                    ? "bg-blue-500 text-white"
                    : "bg-gray-200"
                }`}
                onClick={() =>
                  setWidgetType({ type: "url", content: { url: "" } })
                }
              >
                URL
              </button>
              <button
                type="button"
                className={`flex-1 p-2 text-center text-sm ${
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

            {widgetType.type === "url" ? (
              <div>
                <label htmlFor="url" className={labelClass}>
                  URL
                </label>
                <input
                  type="text"
                  id="url"
                  value={formData.url}
                  onChange={(e) =>
                    setFormData({ ...formData, url: e.target.value })
                  }
                  placeholder="https://example.com"
                  className={`${inputClass} ${
                    validationErrors.url ? "border-red-300" : ""
                  }`}
                  required
                />
                {validationErrors.url && (
                  <p className="text-xs text-red-500 mt-1">
                    {validationErrors.url}
                  </p>
                )}
              </div>
            ) : (
              <div>
                <label htmlFor="html" className={labelClass}>
                  HTML Content
                </label>
                <textarea
                  id="html"
                  value={formData.html}
                  onChange={(e) =>
                    setFormData({ ...formData, html: e.target.value })
                  }
                  placeholder="<div>Hello World</div>"
                  className={`${inputClass} min-h-[80px] ${
                    validationErrors.html ? "border-red-300" : ""
                  }`}
                  rows={4}
                  required
                />
                {validationErrors.html && (
                  <p className="text-xs text-red-500 mt-1">
                    {validationErrors.html}
                  </p>
                )}
              </div>
            )}
          </div>
        );

      case 2:
        return (
          <div className="space-y-4">
            <h3 className="text-sm font-medium mb-4">Appearance</h3>
            <div>
              <label htmlFor="title" className={labelClass}>
                Title
              </label>
              <input
                type="text"
                id="title"
                value={formData.title}
                onChange={(e) =>
                  setFormData({ ...formData, title: e.target.value })
                }
                placeholder="Widget Title"
                className={inputClass}
              />
            </div>

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

            <div className="flex items-center space-x-2">
              <input
                type="checkbox"
                id="transparent"
                checked={formData.transparent}
                onChange={(e) =>
                  setFormData({ ...formData, transparent: e.target.checked })
                }
                className="h-4 w-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
              />
              <label htmlFor="transparent" className="text-sm">
                Transparent Background
              </label>
            </div>
          </div>
        );

      case 3:
        return (
          <div className="space-y-4">
            <h3 className="text-sm font-medium mb-4">Advanced Options</h3>
            {widgetType.type === "url" && (
              <>
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
                        onChange={(e) =>
                          setRefreshInterval(Number(e.target.value))
                        }
                        className={`w-full px-2 py-1 text-sm border rounded focus:outline-none focus:ring-1 focus:ring-blue-500 ${
                          validationErrors.refreshInterval
                            ? "border-red-300"
                            : ""
                        }`}
                      />
                      {validationErrors.refreshInterval && (
                        <p className="text-xs text-red-500 mt-1">
                          {validationErrors.refreshInterval}
                        </p>
                      )}
                    </div>
                  )}
                </div>

                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <div className="text-sm font-medium flex items-center gap-1.5">
                        {scrapeIcon}
                        Scrape Value
                        <div className="group relative">
                          {helpIcon}
                          <div className="absolute left-0 bottom-full mb-2 hidden group-hover:block w-64 p-2 bg-gray-800 text-white text-xs rounded shadow-lg">
                            Use a CSS selector to extract specific content from
                            the webpage. Examples:
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
                          validationErrors.selector ? "border-red-300" : ""
                        }`}
                      />
                      {validationErrors.selector && (
                        <p className="text-xs text-red-500 mt-1">
                          {validationErrors.selector}
                        </p>
                      )}
                    </div>
                  )}
                </div>
              </>
            )}
          </div>
        );

      case 4:
        return (
          <div className="space-y-4">
            <h3 className="text-sm font-medium mb-4">Size and Position</h3>
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
                  className={`${inputClass} ${
                    validationErrors.width ? "border-red-300" : ""
                  }`}
                  min="100"
                />
                {validationErrors.width && (
                  <p className="text-xs text-red-500 mt-1">
                    {validationErrors.width}
                  </p>
                )}
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
                  className={`${inputClass} ${
                    validationErrors.height ? "border-red-300" : ""
                  }`}
                  min="100"
                />
                {validationErrors.height && (
                  <p className="text-xs text-red-500 mt-1">
                    {validationErrors.height}
                  </p>
                )}
              </div>
            </div>
          </div>
        );
    }
  };

  return (
    <div className="max-w-md mx-auto">
      <form onSubmit={handleSubmit} className="p-3 space-y-4">
        <h2 className="font-semibold">Create New Widget</h2>
        {renderStepIndicator()}
        <div className="transition-all duration-300 ease-in-out">
          {renderStepContent()}
        </div>

        <div className="flex justify-between pt-4">
          {currentStep > 1 && (
            <button
              type="button"
              onClick={handleBack}
              className="px-3 py-1 border border-gray-300 rounded-md text-xs font-medium hover:bg-gray-50"
            >
              Back
            </button>
          )}
          {currentStep < 4 ? (
            <button
              type="button"
              onClick={handleNext}
              className="ml-auto px-3 py-1 border border-transparent rounded-md shadow-sm text-xs font-medium text-white bg-blue-600 hover:bg-blue-700"
            >
              Next
            </button>
          ) : (
            <button
              type="submit"
              className="ml-auto px-3 py-1 border border-transparent rounded-md shadow-sm text-xs font-medium text-white bg-blue-600 hover:bg-blue-700"
            >
              Create Widget
            </button>
          )}
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
