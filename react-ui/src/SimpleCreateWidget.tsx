import { useState } from "react";
import { CreateWidgetRequest, Level, ApiError, Modifier } from "./types";
import {
  urlIcon,
  appearanceIcon,
  stackFront,
  stack,
  refreshIcon,
  scrapeIcon,
  helpIcon,
} from "./utils";
import { createWidget } from "./clientInterface";

const steps = [
  { id: 1, title: "Type", icon: urlIcon },
  { id: 2, title: "Appearance", icon: appearanceIcon },
];

const DEFAULT_WIDGET_X = 100;
const DEFAULT_WIDGET_Y = 100;
const DEFAULT_WIDGET_WIDTH = 200;
const DEFAULT_WIDGET_HEIGHT = 200;

interface FormData {
  type: "url" | "file";
  url: string;
  html: string;
  title: string;
  transparent: boolean;
  level: Level;
  autoRefresh: boolean;
  scrapeValue: boolean;
  refreshInterval: number;
  selector: string;
  bounds: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  modifiers: Modifier[];
}

export default function SimpleCreateWidgetForm() {
  const [error, setError] = useState<ApiError | null>(null);
  const [currentStep, setCurrentStep] = useState(1);
  const [validationErrors, setValidationErrors] = useState<
    Record<string, string>
  >({});
  const [formData, setFormData] = useState<FormData>({
    type: "url" as "url" | "file",
    url: "",
    html: "",
    title: "",
    transparent: true,
    level: Level.AlwaysOnTop,
    autoRefresh: false,
    scrapeValue: false,
    refreshInterval: 5,
    selector: "",
    bounds: {
      x: DEFAULT_WIDGET_X,
      y: DEFAULT_WIDGET_Y,
      width: DEFAULT_WIDGET_WIDTH,
      height: DEFAULT_WIDGET_HEIGHT,
    },
    modifiers: [],
  });

  const validateStep = (step: number): boolean => {
    const errors: Record<string, string> = {};

    switch (step) {
      case 1:
        if (formData.type === "url" && !formData.url) {
          errors.url = "URL is required";
        } else if (formData.type === "file" && !formData.html) {
          errors.html = "HTML content is required";
        }
        break;
      case 2:
        // No validation needed for appearance
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

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateStep(currentStep)) return;

    try {
      const request: CreateWidgetRequest = {
        url: formData.url,
        html: formData.html,
        title: formData.title,
        level: formData.level,
        transparent: formData.transparent,
        decorations: false,
        bounds: formData.bounds,
        modifiers: formData.modifiers,
      };

      const response = await createWidget(request);

      if (!response.ok) {
        throw new Error("Failed to create widget");
      }
    } catch (error) {
      console.error("Error creating widget:", error);
      setError({
        message: "Failed to create widget. Please try again.",
        origin: "fetch",
      });
    }
  };

  const handleQuickCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    e.stopPropagation();

    if (!validateStep(1) || !validateStep(currentStep)) return;

    try {
      const request: CreateWidgetRequest = {
        url: formData.url,
        html: formData.html,
        title: formData.title,
        level: formData.level,
        transparent: formData.transparent,
        decorations: false,
        bounds: formData.bounds,
        modifiers: formData.modifiers,
      };

      const response = await createWidget(request);

      if (!response.ok) {
        throw new Error("Failed to create widget");
      }
    } catch (error) {
      console.error("Error creating widget:", error);
      setError({
        message: "Failed to create widget. Please try again.",
        origin: "fetch",
      });
    }
  };

  const handleLevelChange = (level: Level) => {
    setFormData({ ...formData, level });
  };

  const renderStepIndicator = () => (
    <div className="mb-4">
      <div className="flex items-center gap-2">
        {steps.map((step, index) => {
          const isCurrent = step.id === currentStep;
          const isCompleted = step.id < currentStep;

          return (
            <div key={step.id} className="flex items-center">
              <button
                type="button"
                onClick={() => handleStepClick(step.id)}
                className={`
                  flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium
                  transition-all duration-200
                  ${
                    isCurrent
                      ? "bg-blue-500 text-white"
                      : isCompleted
                      ? "bg-blue-50 text-blue-600"
                      : "bg-gray-50 text-gray-500 hover:bg-gray-100"
                  }
                `}
              >
                <div
                  className={`
                  w-5 h-5 rounded-full flex items-center justify-center
                  ${
                    isCurrent
                      ? "bg-white text-blue-500"
                      : isCompleted
                      ? "bg-blue-100 text-blue-600"
                      : "bg-white text-gray-400"
                  }
                `}
                >
                  {isCompleted ? (
                    <svg
                      className="w-3 h-3"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M5 13l4 4L19 7"
                      />
                    </svg>
                  ) : (
                    <span className="text-xs font-medium">{step.id}</span>
                  )}
                </div>
                {step.title}
              </button>
              {index < steps.length - 1 && (
                <div className="w-4 h-[2px] bg-gray-200 mx-1">
                  <div
                    className={`h-full bg-blue-500 transition-all duration-300 ${
                      isCompleted ? "w-full" : "w-0"
                    }`}
                  />
                </div>
              )}
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
                  formData.type === "url"
                    ? "bg-white text-gray-900 shadow-sm"
                    : "text-gray-600 hover:text-gray-900"
                }`}
                onClick={() => setFormData({ ...formData, type: "url" })}
              >
                URL
              </button>
              <button
                type="button"
                className={`flex-1 py-1.5 px-3 rounded-md text-sm font-medium transition-all ${
                  formData.type === "file"
                    ? "bg-white text-gray-900 shadow-sm"
                    : "text-gray-600 hover:text-gray-900"
                }`}
                onClick={() => setFormData({ ...formData, type: "file" })}
              >
                HTML/JS
              </button>
            </div>

            {formData.type === "url" ? (
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
              <div className="flex rounded-lg overflow-hidden border border-gray-200 bg-gray-50">
                <button
                  type="button"
                  className={`flex-1 flex items-center justify-center gap-1 py-2 text-sm font-medium transition-colors focus:outline-none focus:z-10
                    ${
                      formData.level === Level.AlwaysOnBottom
                        ? "bg-white text-blue-600"
                        : "bg-gray-50 text-gray-700 hover:bg-white"
                    }
                  `}
                  style={{ borderRight: "1px solid #e5e7eb" }}
                  onClick={() => handleLevelChange(Level.AlwaysOnBottom)}
                >
                  {stack}
                  <span>Below</span>
                </button>
                <button
                  type="button"
                  className={`flex-1 flex items-center justify-center gap-1 py-2 text-sm font-medium transition-colors focus:outline-none focus:z-10
                    ${
                      formData.level === Level.Normal
                        ? "bg-white text-blue-600"
                        : "bg-gray-50 text-gray-700 hover:bg-white"
                    }
                  `}
                  style={{ borderRight: "1px solid #e5e7eb" }}
                  onClick={() => handleLevelChange(Level.Normal)}
                >
                  Normal
                </button>
                <button
                  type="button"
                  className={`flex-1 flex items-center justify-center gap-1 py-2 text-sm font-medium transition-colors focus:outline-none focus:z-10
                    ${
                      formData.level === Level.AlwaysOnTop
                        ? "bg-white text-blue-600"
                        : "bg-gray-50 text-gray-700 hover:bg-white"
                    }
                  `}
                  onClick={() => handleLevelChange(Level.AlwaysOnTop)}
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

            <div className="bg-gray-50 rounded-lg p-3 space-y-3">
              <div>
                <label className={labelClass}>Position & Size</label>
                <div className="flex items-center gap-1 bg-white rounded-lg border border-gray-200 overflow-hidden">
                  <div className="flex items-center gap-1 px-2 py-1">
                    <span className="text-xs font-medium text-gray-500">X</span>
                    <input
                      type="number"
                      value={formData.bounds.x}
                      onChange={(e) =>
                        setFormData({
                          ...formData,
                          bounds: {
                            ...formData.bounds,
                            x: Number(e.target.value),
                          },
                        })
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
                      value={formData.bounds.y}
                      onChange={(e) =>
                        setFormData({
                          ...formData,
                          bounds: {
                            ...formData.bounds,
                            y: Number(e.target.value),
                          },
                        })
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
                      value={formData.bounds.width}
                      onChange={(e) =>
                        setFormData({
                          ...formData,
                          bounds: {
                            ...formData.bounds,
                            width: Number(e.target.value),
                          },
                        })
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
                      value={formData.bounds.height}
                      onChange={(e) =>
                        setFormData({
                          ...formData,
                          bounds: {
                            ...formData.bounds,
                            height: Number(e.target.value),
                          },
                        })
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
            {formData.type === "url" && (
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
                        checked={formData.autoRefresh}
                        onChange={(e) =>
                          setFormData({
                            ...formData,
                            autoRefresh: e.target.checked,
                          })
                        }
                        className="sr-only peer"
                      />
                      <div className="w-8 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-500"></div>
                    </label>
                  </div>
                  {formData.autoRefresh && (
                    <div className="pl-6">
                      <label className={labelClass}>
                        Refresh Interval (seconds)
                      </label>
                      <input
                        type="number"
                        min="5"
                        max="3600"
                        value={formData.refreshInterval}
                        onChange={(e) =>
                          setFormData({
                            ...formData,
                            refreshInterval: Number(e.target.value),
                          })
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
                        checked={formData.scrapeValue}
                        onChange={(e) =>
                          setFormData({
                            ...formData,
                            scrapeValue: e.target.checked,
                          })
                        }
                        className="sr-only peer"
                      />
                      <div className="w-8 h-4 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-3 after:w-3 after:transition-all peer-checked:bg-blue-500"></div>
                    </label>
                  </div>
                  {formData.scrapeValue && (
                    <div className="pl-6">
                      <label className={labelClass}>CSS Selector</label>
                      <input
                        type="text"
                        value={formData.selector}
                        onChange={(e) =>
                          setFormData({ ...formData, selector: e.target.value })
                        }
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

  return (
    <div className="max-w-[95%] w-[460px] mx-auto">
      <form onSubmit={handleSubmit} className="p-2">
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
            {currentStep === 1 && (
              <button
                type="button"
                onClick={handleQuickCreate}
                className={buttonClass.quick}
              >
                Quick Create
              </button>
            )}
            {currentStep < 2 ? (
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
