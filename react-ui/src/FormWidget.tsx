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

export default function WidgetForm() {
  // const [currFormValues, setCurrFormValues] =
  //   useState<CreateWidgetRequest>(defaultValues);
  const [error, setError] = useState<ApiError | null>(null);
  const [widgetType, setWidgetType] = useState<WidgetType>({
    type: "url",
    content: { url: "" },
  });

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const formData = new FormData(event.currentTarget);
    const data: CreateWidgetRequest = {
      url: (formData.get("url") as string) ?? "",
      html: (formData.get("html") as string) ?? "",
      title: (formData.get("title") as string) ?? "",
      level:
        (formData.get("level") as WidgetConfiguration["level"]) ?? Level.Normal,
      transparent: formData.get("transparent") === "true",
    };
    console.log("Form submitted:", data);

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
    } else {
      const error = await res.json();
      setError(error);
    }
  };

  const handleReset = (event: React.FormEvent<HTMLFormElement>) => {
    event.currentTarget.reset();
  };

  const inputClass =
    "w-full px-4 h-8 bg-white border-[3px] border-black text-lg focus:outline-none appearance-none";
  const labelClass =
    "block w-full bg-[#FF90BC] h-8 leading-7 border-x-[3px] border-t-[3px] border-black text-center font-black text-lg uppercase tracking-wider";

  return (
    <div className="p-2 max-w-md mx-auto">
      <div className="shadow-[6px_6px_0px_0px_rgba(0,0,0,1)]">
        <form onSubmit={handleSubmit} onReset={handleReset}>
          {/* URL Field */}
          <div className="flex justify-between w-full">
            <button
              type="button"
              className={`w-full transition-colors ${
                widgetType.type === "url"
                  ? "bg-[#98EECC] hover:bg-[#7DCCAA]"
                  : "bg-[#FF90BC] hover:bg-[#FF7FAD] opacity-75"
              }`}
              onClick={() =>
                setWidgetType({ type: "url", content: { url: "" } })
              }
            >
              <label
                htmlFor="url"
                className={`block w-full h-8 leading-7 border-x-[3px] border-t-[3px] border-black text-center font-black text-lg uppercase tracking-wider cursor-pointer ${
                  widgetType.type === "url" ? "text-black" : "text-gray-700"
                }`}
              >
                URL
              </label>
            </button>

            <button
              type="button"
              className={`w-full transition-colors ${
                widgetType.type === "file"
                  ? "bg-[#98EECC] hover:bg-[#7DCCAA]"
                  : "bg-[#FF90BC] hover:bg-[#FF7FAD] opacity-75"
              }`}
              onClick={() =>
                setWidgetType({ type: "file", content: { html: "" } })
              }
            >
              <label
                htmlFor="url"
                className={`block w-full h-8 leading-7 border-x-[3px] border-t-[3px] border-black text-center font-black text-lg uppercase tracking-wider cursor-pointer ${
                  widgetType.type === "file" ? "text-black" : "text-gray-700"
                }`}
              >
                HTML/JS
              </label>
            </button>
          </div>
          {widgetType.type === "url" && (
            <div>
              <input
                type="text"
                id="url"
                name="url"
                defaultValue={defaultValues.url}
                placeholder="https://example.com"
                className={`${inputClass} border-b-[3px]`}
                required
              />
            </div>
          )}
          {widgetType.type === "file" && (
            <div>
              <textarea
                id="file"
                name="file"
                defaultValue={defaultValues.html}
                placeholder="<div>Hello World</div>"
                className={`${inputClass} border-b-[3px]`}
                rows={10}
                cols={50}
                required
              />
            </div>
          )}

          <div>
            <label htmlFor="title" className={labelClass}>
              Title
            </label>
            <input
              type="text"
              id="title"
              name="title"
              defaultValue={defaultValues.title}
              placeholder="Title"
              className={`${inputClass} border-b-[3px]`}
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
              className={`${inputClass} border-b-[3px] cursor-pointer`}
            >
              <option value="normal">Normal</option>
              <option value="alwaysontop">Always on Top</option>
              <option value="alwaysonbottom">Always on Bottom</option>
            </select>
          </div>

          {/* Transparent Field */}
          <div className="flex justify-between w-full border-b-[3px] border-black">
            <label htmlFor="transparent" className={labelClass}>
              Transparent
            </label>
            <div className={`${inputClass} flex items-center justify-center`}>
              <input
                type="checkbox"
                id="transparent"
                name="transparent"
                defaultChecked={defaultValues.transparent}
                className="w-6 h-6 border-[3px] border-black bg-white checked:bg-[#98EECC] appearance-none cursor-pointer"
              />
            </div>
          </div>

          {/* Form Actions */}
          <div className="flex w-full bg-[#FF90BC] border-b-[3px] border-black">
            <button
              type="reset"
              className="h-10 w-1/3 text-lg font-black bg-[#98EECC] border-l-[3px] border-black uppercase hover:bg-[#7DCCAA] active:bg-[#98EECC] transition-colors"
            >
              Reset
            </button>
            <button
              type="submit"
              className="h-10 w-2/3 text-lg font-black bg-[#A7D2CB] border-l-[3px] border-black uppercase hover:bg-[#86B1AA] active:bg-[#A7D2CB] transition-colors"
            >
              Create
            </button>
          </div>
        </form>
      </div>
      {error && (
        <div className="text-red-500 h-12 text-center text-lg flex flex-col justify-center items-center">
          <label>Error: {error.message}</label>
        </div>
      )}
    </div>
  );
}
