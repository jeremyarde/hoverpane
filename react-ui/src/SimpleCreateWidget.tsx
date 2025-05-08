import { useState } from "react";
import { CreateWidgetRequest, Level, WidgetType } from "./types";
import {
  urlIcon,
  appearanceIcon,
  settingsIcon,
  stack,
  stackFront,
  refreshIcon,
  scrapeIcon,
  helpIcon,
} from "./utils";

const steps = [
  { id: 1, title: "Type", icon: urlIcon },
  { id: 2, title: "Appearance", icon: appearanceIcon },
  { id: 3, title: "Advanced", icon: settingsIcon },
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
    transparent: true,
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

  const isStepValid = (step: number): boolean => {
    const isValid = validateStep(step);
    return isValid;
  };

  const handleStepClick = (stepId: number) => {
    // If trying to go forward, validate current step first
    if (stepId > currentStep && !isStepValid(currentStep)) {
      return;
    }
    setCurrentStep(stepId);
  };

  const handleNext = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (isStepValid(currentStep)) {
      setCurrentStep((prev) => Math.min(prev + 1, 4));
    }
  };

  const handleBack = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
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

  const handleQuickCreate = async (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    // Validate both step 1 and current step if we're on step 2
    if (!validateStep(1) || (currentStep === 2 && !validateStep(2))) {
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
      bounds: {
        x: DEFAULT_WIDGET_X,
        y: DEFAULT_WIDGET_Y,
        width: DEFAULT_WIDGET_WIDTH,
        height: DEFAULT_WIDGET_HEIGHT,
      },
    };

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

  const renderStepIndicator = () => (
    <div className="mb-4">
      <div className="relative flex items-center justify-between mb-3">
        {/* Progress bar (continuous) */}
        <div
          className="absolute top-1/2 left-0 right-0 h-0.5 bg-gray-200 z-0"
          style={{ transform: "translateY(-50%)" }}
        />
        <div
          className="absolute top-1/2 left-0 h-0.5 bg-blue-500 z-10 transition-all duration-300"
          style={{
            width: `${((currentStep - 1) / (steps.length - 1)) * 100}%`,
            transform: "translateY(-50%)",
          }}
        />
        {steps.map((step) => {
          const isCurrent = step.id === currentStep;
          return (
            <div
              key={step.id}
              className="relative z-20 flex flex-col items-center flex-1"
            >
              <button
                type="button"
                onClick={() => handleStepClick(step.id)}
                className={`
                  w-7 h-7 rounded-full flex items-center justify-center mb-1 border transition-all duration-200 text-base
                  ${
                    isCurrent
                      ? "border-blue-600 text-blue-600 shadow-sm bg-white"
                      : "border-gray-300 text-gray-400 bg-white hover:border-gray-400 hover:text-gray-600"
                  }
                  cursor-pointer
                `}
              >
                {step.icon}
              </button>
              <span
                className={`
                  text-[10px] font-medium cursor-pointer hover:text-gray-600
                  ${isCurrent ? "text-blue-700" : "text-gray-400"}
                `}
                onClick={() => handleStepClick(step.id)}
              >
                {step.title}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );

  const renderStepContent = () => {
    switch (currentStep) {
      case 1:
        return (
          <div className="space-y-3">
            <div className="p-1 bg-gray-100 rounded-lg flex">
              <button
                type="button"
                className={`flex-1 py-1.5 px-3 rounded-md text-sm font-medium transition-all ${
                  widgetType.type === "url"
                    ? "bg-white text-gray-900 shadow-sm"
                    : "text-gray-600 hover:text-gray-900"
                }`}
                onClick={() =>
                  setWidgetType({ type: "url", content: { url: "" } })
                }
              >
                URL
              </button>
              <button
                type="button"
                className={`flex-1 py-1.5 px-3 rounded-md text-sm font-medium transition-all ${
                  widgetType.type === "file"
                    ? "bg-white text-gray-900 shadow-sm"
                    : "text-gray-600 hover:text-gray-900"
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
                    validationErrors.url
                      ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                      : ""
                  }`}
                  required
                />
                {validationErrors.url && (
                  <p className="text-xs text-red-500 mt-0.5">
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
                    validationErrors.html
                      ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                      : ""
                  }`}
                  rows={4}
                  required
                />
                {validationErrors.html && (
                  <p className="text-xs text-red-500 mt-0.5">
                    {validationErrors.html}
                  </p>
                )}
              </div>
            )}
          </div>
        );

      case 2:
        return (
          <div className="space-y-3">
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
              <div className="p-1 bg-gray-100 rounded-lg flex">
                <button
                  type="button"
                  className={`flex-1 py-1.5 px-3 rounded-md text-sm font-medium transition-all flex items-center justify-center gap-2 ${
                    selectedLevel === Level.AlwaysOnBottom
                      ? "bg-white text-gray-900 shadow-sm"
                      : "text-gray-600 hover:text-gray-900"
                  }`}
                  onClick={() => setSelectedLevel(Level.AlwaysOnBottom)}
                >
                  {stack}
                  <span>Bottom</span>
                </button>
                <button
                  type="button"
                  className={`flex-1 py-1.5 px-3 rounded-md text-sm font-medium transition-all ${
                    selectedLevel === Level.Normal
                      ? "bg-white text-gray-900 shadow-sm"
                      : "text-gray-600 hover:text-gray-900"
                  }`}
                  onClick={() => setSelectedLevel(Level.Normal)}
                >
                  Normal
                </button>
                <button
                  type="button"
                  className={`flex-1 py-1.5 px-3 rounded-md text-sm font-medium transition-all flex items-center justify-center gap-2 ${
                    selectedLevel === Level.AlwaysOnTop
                      ? "bg-white text-gray-900 shadow-sm"
                      : "text-gray-600 hover:text-gray-900"
                  }`}
                  onClick={() => setSelectedLevel(Level.AlwaysOnTop)}
                >
                  {stackFront}
                  <span>Top</span>
                </button>
              </div>
            </div>

            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <div className="text-xs font-medium flex items-center gap-1.5">
                  Transparent Background
                </div>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  id="transparent"
                  checked={formData.transparent}
                  onChange={(e) =>
                    setFormData({ ...formData, transparent: e.target.checked })
                  }
                  className="sr-only peer"
                />
                <div className="w-8 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-500"></div>
              </label>
            </div>
          </div>
        );

      case 3:
        return (
          <div className="space-y-4">
            <div className="bg-gray-50 rounded-lg p-3 space-y-3">
              <div>
                <label className={labelClass}>Position & Size</label>
                <div className="flex items-center gap-1 bg-white rounded-lg border border-gray-200 overflow-hidden">
                  <div className="flex items-center gap-1 px-2 py-1">
                    <span className="text-xs font-medium text-gray-500">X</span>
                    <input
                      type="number"
                      value={bounds.x}
                      onChange={(e) =>
                        setBounds({ ...bounds, x: Number(e.target.value) })
                      }
                      className={`w-14 text-xs border-0 p-0 focus:ring-0 ${
                        validationErrors.x ? "text-red-500" : ""
                      }`}
                      min="0"
                    />
                  </div>
                  <div className="h-4 w-px bg-gray-200" />
                  <div className="flex items-center gap-1 px-2 py-1">
                    <span className="text-xs font-medium text-gray-500">Y</span>
                    <input
                      type="number"
                      value={bounds.y}
                      onChange={(e) =>
                        setBounds({ ...bounds, y: Number(e.target.value) })
                      }
                      className={`w-14 text-xs border-0 p-0 focus:ring-0 ${
                        validationErrors.y ? "text-red-500" : ""
                      }`}
                      min="0"
                    />
                  </div>
                  <div className="h-4 w-px bg-gray-200" />
                  <div className="flex items-center gap-1 px-2 py-1">
                    <span className="text-xs font-medium text-gray-500">W</span>
                    <input
                      type="number"
                      value={bounds.width}
                      onChange={(e) =>
                        setBounds({ ...bounds, width: Number(e.target.value) })
                      }
                      className={`w-14 text-xs border-0 p-0 focus:ring-0 ${
                        validationErrors.width ? "text-red-500" : ""
                      }`}
                      min="100"
                    />
                  </div>
                  <div className="h-4 w-px bg-gray-200" />
                  <div className="flex items-center gap-1 px-2 py-1">
                    <span className="text-xs font-medium text-gray-500">H</span>
                    <input
                      type="number"
                      value={bounds.height}
                      onChange={(e) =>
                        setBounds({ ...bounds, height: Number(e.target.value) })
                      }
                      className={`w-14 text-xs border-0 p-0 focus:ring-0 ${
                        validationErrors.height ? "text-red-500" : ""
                      }`}
                      min="100"
                    />
                  </div>
                </div>
                <div className="flex gap-2 mt-1">
                  {validationErrors.width && (
                    <p className="text-xs text-red-500">Min width: 100px</p>
                  )}
                  {validationErrors.height && (
                    <p className="text-xs text-red-500">Min height: 100px</p>
                  )}
                </div>
              </div>
            </div>
            {widgetType.type === "url" && (
              <>
                <div className="space-y-2 pt-2">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <div className="text-xs font-medium flex items-center gap-1.5">
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
                      <div className="w-8 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-500"></div>
                    </label>
                  </div>
                  {autoRefresh && (
                    <div className="pl-6">
                      <label className={labelClass}>
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
                        className={`${inputClass} ${
                          validationErrors.refreshInterval
                            ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                            : ""
                        }`}
                      />
                      {validationErrors.refreshInterval && (
                        <p className="text-xs text-red-500 mt-0.5">
                          {validationErrors.refreshInterval}
                        </p>
                      )}
                    </div>
                  )}
                </div>

                <div className="space-y-2">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <div className="text-xs font-medium flex items-center gap-1.5">
                        {scrapeIcon}
                        Scrape Value
                        <div className="group relative">
                          {helpIcon}
                          <div className="absolute left-0 bottom-full mb-2 hidden group-hover:block w-56 p-2 bg-gray-800 text-white text-[11px] rounded shadow-lg">
                            Use a CSS selector to extract specific content from
                            the webpage.
                            <ul className="mt-1 ml-2 list-disc text-[10px]">
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
                      <div className="w-8 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-500"></div>
                    </label>
                  </div>
                  {scrapeValue && (
                    <div className="pl-6">
                      <label className={labelClass}>CSS Selector</label>
                      <input
                        type="text"
                        value={selector}
                        onChange={(e) => setSelector(e.target.value)}
                        placeholder="#price, .value, etc."
                        className={`${inputClass} ${
                          validationErrors.selector
                            ? "border-red-300 focus:border-red-500 focus:ring-red-500"
                            : ""
                        }`}
                      />
                      {validationErrors.selector && (
                        <p className="text-xs text-red-500 mt-0.5">
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
    }
  };

  const inputClass =
    "w-full px-2 py-1.5 text-xs border border-gray-200 rounded bg-white focus:border-blue-500 focus:ring-1 focus:ring-blue-500 outline-none transition-colors placeholder-gray-400";
  const labelClass = "block text-[11px] font-medium mb-0.5 text-gray-500";
  const buttonClass = {
    primary:
      "px-3 py-1.5 text-xs font-medium text-white bg-blue-500 rounded border border-transparent hover:bg-blue-600 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1",
    secondary:
      "px-3 py-1.5 text-xs font-medium text-gray-700 bg-white rounded border border-gray-200 hover:bg-gray-50 hover:border-gray-300 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1",
    quick:
      "px-3 py-1.5 text-xs font-medium text-blue-600 bg-blue-50 rounded border border-blue-200 hover:bg-blue-100 hover:border-blue-300 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1",
    tab: (active: boolean) =>
      `flex-1 py-2 text-sm font-medium transition-colors ${
        active ? "text-blue-600" : "text-gray-500 hover:text-gray-700"
      }`,
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      e.preventDefault();
    }
  };

  return (
    <div className="max-w-[95%] w-[460px] mx-auto">
      <form onSubmit={handleSubmit} className="p-2" onKeyDown={handleKeyDown}>
        <h2 className="font-semibold text-base text-gray-800">
          Create New Widget
        </h2>
        {renderStepIndicator()}
        <div className="transition-all duration-300 ease-in-out space-y-4">
          {renderStepContent()}
        </div>
        <div className="flex justify-end gap-2 pt-4">
          {currentStep > 1 && (
            <button
              type="button"
              onClick={handleBack}
              className={buttonClass.secondary}
            >
              Back
            </button>
          )}
          <div className="flex gap-2">
            {(currentStep === 1 || currentStep === 2) && (
              <button
                type="button"
                onClick={handleQuickCreate}
                className={buttonClass.quick}
              >
                Quick Create
              </button>
            )}
            {currentStep < 3 ? (
              <button
                type="button"
                onClick={handleNext}
                className={buttonClass.primary}
              >
                Next
              </button>
            ) : (
              <button type="submit" className={buttonClass.primary}>
                Create Widget
              </button>
            )}
          </div>
        </div>
      </form>
      {error && (
        <div className="mt-2 px-2 py-1.5 bg-red-50 border border-red-200 text-red-600 rounded-md text-xs flex items-start gap-2 relative">
          <button
            className="absolute top-0.5 right-1.5 text-red-400 hover:text-red-600 focus:outline-none"
            aria-label="Dismiss error"
            onClick={() => setError(null)}
            type="button"
          >
            ×
          </button>
          <div className="pr-4">
            <span className="font-medium">Error:</span> {error.message}
          </div>
        </div>
      )}
    </div>
  );
}
