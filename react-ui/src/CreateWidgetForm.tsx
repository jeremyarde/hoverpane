import { useState } from "react";
import {
  CreateWidgetRequest,
  Level,
  WidgetConfiguration,
  WidgetType,
} from "./types";

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
};

export default function CreateWidgetForm() {
  const [error, setError] = useState<ApiError | null>(null);
  const [widgetType, setWidgetType] = useState<WidgetType>({
    type: "url",
    content: { url: "" },
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
      level:
        (formData.get("level") as WidgetConfiguration["level"]) ?? Level.Normal,
      transparent: formData.get("transparent") === "on", // Checkbox value is 'on' when checked
    };
    console.log("Form submitted:", data);

    try {
      const res = await fetch("http://127.0.0.1:3000/widgets", {
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
        event.currentTarget.reset(); // Reset form on success
        setWidgetType({ type: "url", content: { url: "" } }); // Reset type state
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
    event.currentTarget.reset();
    setError(null);
    setWidgetType({ type: "url", content: { url: "" } });
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
            placeholder="My Awesome Widget"
            className={inputClass}
            required
          />
        </div>

        {/* Window Level Field */}
        <div>
          <label htmlFor="level" className={labelClass}>
            Window Level
          </label>
          <select
            id="level"
            name="level"
            defaultValue={defaultValues.level}
            className={inputClass}
          >
            <option value={Level.Normal}>Normal</option>
            <option value={Level.AlwaysOnTop}>Always on Top</option>
            <option value={Level.AlwaysOnBottom}>Always on Bottom</option>
          </select>
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
