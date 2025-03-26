import { AnyFieldApi, useForm } from "@tanstack/react-form";
import { z } from "zod";
import { useState } from "react";

// Define the schema with Zod
const widgetSchema = z.object({
  title: z.string().optional(),
  widget_type: z.enum(["source", "url", "file"]),
  level: z.enum(["normal", "alwaysontop", "alwaysonbottom"]),
  url: z.string().optional(),
  selector: z.string().optional(),
  refresh_interval: z.number().optional(),
  html: z.string().optional(),
  position: z.tuple([z.number().min(0), z.number().min(0)]),
  size: z.tuple([z.number().min(0), z.number().min(0)]),
  background_color: z.tuple([
    z.number().min(0).max(255),
    z.number().min(0).max(255),
    z.number().min(0).max(255),
    z.number().refine((n) => n >= 0 && n <= 1, {
      message: "Alpha must be between 0 and 1",
    }),
  ]),
  resizeable: z.boolean().optional(),
  movable: z.boolean().optional(),
});

// First, get the type from the schema
// type WidgetFormValues = z.infer<typeof widgetSchema>;

function FieldInfo({ field }: { field: AnyFieldApi }) {
  return (
    <>
      {(field.state.meta.isTouched && field.state.meta.errors.length) ||
      field.state.meta.isValidating ? (
        <div
          className="absolute -top-2 right-2 z-10 text-[10px] font-medium px-1.5 py-0.5 rounded-full transform transition-all duration-200"
          style={{
            backgroundColor: field.state.meta.errors.length
              ? "rgba(252, 165, 165, 0.2)"
              : "rgba(147, 197, 253, 0.2)",
            color: field.state.meta.errors.length ? "#ef4444" : "#3b82f6",
          }}
        >
          {field.state.meta.isValidating
            ? "•••"
            : field.state.meta.errors.join(", ")}
        </div>
      ) : null}
    </>
  );
}

declare global {
  interface Window {
    onRustMessage: (element: string) => void;
    ipc: {
      postMessage: (message: string) => void;
    };
    WIDGET_ID: string;
    WINDOW_ID: string;
  }
}

export const NewWidgetForm = () => {
  const [widgetType, setWidgetType] = useState<"source" | "url" | "file">(
    "source"
  );
  const [showSizeAdvanced, setShowSizeAdvanced] = useState(false);
  const [showPositionAdvanced, setShowPositionAdvanced] = useState(false);

  const form = useForm({
    defaultValues: {
      title: "",
      widget_type: "source",
      level: "normal",
      url: "",
      selector: "",
      refresh_interval: 240,
      html: "",
      size: [400, 300],
      position: [0, 0],
      background_color: [255, 255, 255, 1],
      resizeable: true,
      movable: true,
    },
    validators: {
      onChange: (values) => {
        const result = widgetSchema.safeParse(values);
        return result.success ? undefined : { error: result.error };
      },
    },
  });

  // Update the widget type options in the UI
  const widgetTypes = [
    { type: "url", icon: "🔗", label: "URL" },
    { type: "file", icon: "📄", label: "File" },
    { type: "source", icon: "📊", label: "Source" },
  ] as const;

  return (
    <div className="max-w-md mx-auto p-3 bg-white rounded-xl shadow-lg border border-gray-100">
      <form
        className="space-y-3"
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
      >
        {/* Widget Type Selection */}
        <div className="grid grid-cols-3 gap-2">
          <form.Field name="widget_type">
            {(field) => (
              <>
                {widgetTypes.map(({ type, icon, label }) => (
                  <button
                    key={type}
                    type="button"
                    onClick={() => {
                      console.log("clicked", type);
                      field.handleChange(type);
                      setWidgetType(type); // Update local state too
                    }}
                    className={`p-2 rounded-lg text-center transition-all ${
                      widgetType === type // Use local state for UI
                        ? "bg-blue-500 text-white ring-2 ring-blue-300"
                        : "bg-gray-50 text-gray-600 hover:bg-gray-100"
                    }`}
                  >
                    <div className="text-lg mb-1">{icon}</div>
                    <div className="text-[10px] font-medium">{label}</div>
                  </button>
                ))}
              </>
            )}
          </form.Field>
        </div>

        {/* Title Field */}
        <form.Field name="title">
          {(field) => (
            <div className="relative">
              <input
                placeholder="Widget Title"
                value={field.state.value}
                onChange={(e) => field.handleChange(e.target.value)}
                className="w-full h-8 px-3 text-sm bg-gray-50 rounded-lg focus:outline-none focus:ring-1 focus:ring-blue-500 placeholder:text-gray-400"
              />
              <FieldInfo field={field} />
            </div>
          )}
        </form.Field>

        {/* Source Widget Fields */}
        {widgetType === "source" && (
          <div className="space-y-2">
            <form.Field name="url">
              {(field) => (
                <div className="relative">
                  <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                    🔗
                  </div>
                  <input
                    required
                    type="url"
                    placeholder="URL (required)"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                    className="w-full h-7 pl-7 pr-2 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                  <FieldInfo field={field} />
                </div>
              )}
            </form.Field>

            <form.Field name="selector">
              {(field) => (
                <div className="relative">
                  <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                    🎯
                  </div>
                  <input
                    required
                    placeholder="CSS Selector (required)"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                    className="w-full h-7 pl-7 pr-2 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                  <FieldInfo field={field} />
                </div>
              )}
            </form.Field>
          </div>
        )}

        {/* File Widget Fields */}
        {widgetType === "file" && (
          <div className="relative">
            <form.Field name="html">
              {(field) => (
                <div className="relative">
                  <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                    📄
                  </div>
                  <input
                    required
                    type="file"
                    accept=".html,.htm"
                    onChange={(e) =>
                      field.handleChange(e.target.files?.[0]?.name || "")
                    }
                    className="w-full h-7 pl-7 pr-2 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                  <FieldInfo field={field} />
                </div>
              )}
            </form.Field>
          </div>
        )}

        {/* URL Widget Fields */}
        {widgetType === "url" && (
          <div className="relative">
            <form.Field name="url">
              {(field) => (
                <div className="relative">
                  <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                    🔗
                  </div>
                  <input
                    required
                    type="url"
                    placeholder="URL (required)"
                    value={field.state.value}
                    onChange={(e) => field.handleChange(e.target.value)}
                    className="w-full h-7 pl-7 pr-2 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                  />
                  <FieldInfo field={field} />
                </div>
              )}
            </form.Field>
          </div>
        )}

        {/* Bottom controls bar */}
        <div className="flex flex-col sm:flex-row items-stretch gap-2 pt-2 border-t border-gray-100">
          <div className="flex flex-col sm:flex-row flex-1 gap-2">
            {/* Size and Position Group */}
            <div className="flex flex-wrap items-center gap-2 p-1.5 bg-gray-50/50 rounded-lg border border-gray-100 flex-1">
              <div className="flex items-center text-[10px] font-medium text-gray-500 px-1 w-full sm:w-auto">
                Layout
              </div>
              {/* Size Field */}
              <form.Field name="size">
                {(field) => (
                  <div className="relative flex-shrink-0 group flex-1 min-w-[120px]">
                    <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                      📐
                    </div>
                    <div className="absolute -top-8 left-0 invisible group-hover:visible bg-gray-800 text-white text-[10px] px-2 py-1 rounded whitespace-nowrap z-10">
                      Widget Size (Width × Height)
                    </div>
                    <button
                      type="button"
                      onClick={() => setShowSizeAdvanced(!showSizeAdvanced)}
                      className="w-full h-7 px-2 pl-7 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 whitespace-nowrap text-left"
                    >
                      {field.state.value[0]}×{field.state.value[1]}
                    </button>
                    {showSizeAdvanced && (
                      <div className="absolute top-full left-0 mt-1 p-2 bg-white rounded-lg shadow-lg border border-gray-100 z-20 flex gap-1">
                        <input
                          type="number"
                          min="0"
                          placeholder="W"
                          title="Width in pixels"
                          value={field.state.value[0]}
                          onChange={(e) =>
                            field.handleChange([
                              Number(e.target.value),
                              field.state.value[1],
                            ])
                          }
                          className="w-16 h-7 px-2 text-xs bg-gray-50 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                        />
                        <input
                          type="number"
                          min="0"
                          placeholder="H"
                          title="Height in pixels"
                          value={field.state.value[1]}
                          onChange={(e) =>
                            field.handleChange([
                              field.state.value[0],
                              Number(e.target.value),
                            ])
                          }
                          className="w-16 h-7 px-2 text-xs bg-gray-50 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                        />
                      </div>
                    )}
                    <FieldInfo field={field} />
                  </div>
                )}
              </form.Field>

              {/* Position Field */}
              <form.Field name="position">
                {(field) => (
                  <div className="relative flex-shrink-0 group flex-1 min-w-[120px]">
                    <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                      📍
                    </div>
                    <div className="absolute -top-8 left-0 invisible group-hover:visible bg-gray-800 text-white text-[10px] px-2 py-1 rounded whitespace-nowrap z-10">
                      Widget Position (X, Y coordinates)
                    </div>
                    <button
                      type="button"
                      onClick={() =>
                        setShowPositionAdvanced(!showPositionAdvanced)
                      }
                      className="w-full h-7 px-2 pl-7 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 whitespace-nowrap text-left"
                    >
                      {field.state.value[0]}, {field.state.value[1]}
                    </button>
                    {showPositionAdvanced && (
                      <div className="absolute top-full left-0 mt-1 p-2 bg-white rounded-lg shadow-lg border border-gray-100 z-20 flex gap-1">
                        <input
                          type="number"
                          min="0"
                          placeholder="X"
                          title="X coordinate in pixels"
                          value={field.state.value[0]}
                          onChange={(e) =>
                            field.handleChange([
                              Number(e.target.value),
                              field.state.value[1],
                            ])
                          }
                          className="w-16 h-7 px-2 text-xs bg-gray-50 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                        />
                        <input
                          type="number"
                          min="0"
                          placeholder="Y"
                          title="Y coordinate in pixels"
                          value={field.state.value[1]}
                          onChange={(e) =>
                            field.handleChange([
                              field.state.value[0],
                              Number(e.target.value),
                            ])
                          }
                          className="w-16 h-7 px-2 text-xs bg-gray-50 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                        />
                      </div>
                    )}
                    <FieldInfo field={field} />
                  </div>
                )}
              </form.Field>
            </div>

            {/* Behavior Group */}
            <div className="flex flex-wrap items-center gap-2 p-1.5 bg-gray-50/50 rounded-lg border border-gray-100 flex-1">
              <div className="flex items-center text-[10px] font-medium text-gray-500 px-1 w-full sm:w-auto">
                Behavior
              </div>
              {/* Show refresh interval only for types that need it */}
              {(widgetType === "source" || widgetType === "url") && (
                <form.Field name="refresh_interval">
                  {(field) => (
                    <div className="relative flex-shrink-0 group flex-1 min-w-[120px]">
                      <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                        ⏱️
                      </div>
                      <div className="absolute -top-8 left-0 invisible group-hover:visible bg-gray-800 text-white text-[10px] px-2 py-1 rounded whitespace-nowrap z-10">
                        Refresh Interval (seconds)
                      </div>
                      <input
                        type="number"
                        min="0"
                        required={
                          widgetType === "url" || widgetType === "source"
                        }
                        placeholder={
                          widgetType === "url" ? "Optional" : "Required"
                        }
                        title="Refresh interval in seconds"
                        value={field.state.value}
                        onChange={(e) =>
                          field.handleChange(Number(e.target.value))
                        }
                        className="w-full h-7 pl-7 pr-2 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                      />
                      <FieldInfo field={field} />
                    </div>
                  )}
                </form.Field>
              )}

              <form.Field name="level">
                {(field) => (
                  <div className="relative flex-shrink-0 group flex-1 min-w-[120px]">
                    <div className="absolute -top-8 left-0 invisible group-hover:visible bg-gray-800 text-white text-[10px] px-2 py-1 rounded whitespace-nowrap z-10">
                      Widget Layer Level
                    </div>
                    <select
                      value={field.state.value}
                      onChange={(e) =>
                        field.handleChange(
                          e.target.value as
                            | "normal"
                            | "alwaysontop"
                            | "alwaysonbottom"
                        )
                      }
                      className="w-full h-7 px-2 text-xs bg-white rounded-md appearance-none focus:outline-none focus:ring-1 focus:ring-blue-500"
                    >
                      <option value="normal">Normal</option>
                      <option value="alwaysontop">Always on Top</option>
                      <option value="alwaysonbottom">Always on Bottom</option>
                    </select>
                  </div>
                )}
              </form.Field>

              {/* Movement Controls */}
              <div className="flex gap-2 flex-1 min-w-[120px]">
                <form.Field name="resizeable">
                  {(field) => (
                    <div className="relative flex items-center gap-1.5">
                      <input
                        type="checkbox"
                        checked={field.state.value}
                        onChange={(e) => field.handleChange(e.target.checked)}
                        className="w-3.5 h-3.5 rounded border-gray-300 text-blue-500 focus:ring-1 focus:ring-blue-500"
                      />
                      <label className="text-[10px] text-gray-600">
                        Resizeable
                      </label>
                    </div>
                  )}
                </form.Field>

                <form.Field name="movable">
                  {(field) => (
                    <div className="relative flex items-center gap-1.5">
                      <input
                        type="checkbox"
                        checked={field.state.value}
                        onChange={(e) => field.handleChange(e.target.checked)}
                        className="w-3.5 h-3.5 rounded border-gray-300 text-blue-500 focus:ring-1 focus:ring-blue-500"
                      />
                      <label className="text-[10px] text-gray-600">
                        Movable
                      </label>
                    </div>
                  )}
                </form.Field>
              </div>
            </div>

            {/* Appearance Group */}
            <div className="flex flex-wrap items-center gap-2 p-1.5 bg-gray-50/50 rounded-lg border border-gray-100 flex-1">
              <div className="flex items-center text-[10px] font-medium text-gray-500 px-1 w-full sm:w-auto">
                Appearance
              </div>
              <form.Field name="background_color">
                {(field) => (
                  <div className="relative flex-shrink-0 group flex-1 min-w-[120px]">
                    <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                      🎨
                    </div>
                    <div className="absolute -top-8 left-0 invisible group-hover:visible bg-gray-800 text-white text-[10px] px-2 py-1 rounded whitespace-nowrap z-10">
                      Background Color (RGBA)
                    </div>
                    <button
                      type="button"
                      onClick={() => {
                        const [r, g, b, a] = field.state.value;
                        const color = `rgba(${r}, ${g}, ${b}, ${a})`;
                        const input = document.createElement("input");
                        input.type = "color";
                        input.value = rgbToHex(r, g, b);
                        input.addEventListener("input", (e) => {
                          const [r, g, b] = hexToRgb(
                            (e.target as HTMLInputElement).value
                          );
                          field.handleChange([r, g, b, field.state.value[3]]);
                        });
                        input.click();
                      }}
                      className="w-full h-7 px-2 pl-7 text-xs bg-white rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 whitespace-nowrap text-left flex items-center gap-2"
                    >
                      <div
                        className="w-4 h-4 rounded-sm border border-gray-200"
                        style={{
                          backgroundColor: `rgba(${field.state.value.join(
                            ", "
                          )})`,
                        }}
                      />
                      <span>
                        rgba(
                        {field.state.value
                          .map((v, i) =>
                            i === 3 ? v.toFixed(2) : Math.round(v)
                          )
                          .join(", ")}
                        )
                      </span>
                    </button>
                    <div className="absolute right-2 top-1.5">
                      <input
                        type="range"
                        min="0"
                        max="1"
                        step="0.01"
                        value={field.state.value[3]}
                        onChange={(e) => {
                          const newAlpha = parseFloat(e.target.value);
                          field.handleChange([
                            ...field.state.value.slice(0, 3),
                            newAlpha,
                          ]);
                        }}
                        className="w-12 h-4 appearance-none bg-transparent [&::-webkit-slider-runnable-track]:h-0.5 [&::-webkit-slider-runnable-track]:rounded [&::-webkit-slider-runnable-track]:bg-gray-200 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2 [&::-webkit-slider-thumb]:h-2 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-blue-500 [&::-webkit-slider-thumb]:mt-[-3px]"
                      />
                    </div>
                    <FieldInfo field={field} />
                  </div>
                )}
              </form.Field>
            </div>
          </div>

          <form.Subscribe
            selector={(state) => [state.canSubmit, state.isSubmitting]}
          >
            {([canSubmit, isSubmitting]) => (
              <button
                type="submit"
                disabled={!canSubmit}
                className="h-7 px-4 bg-gradient-to-r from-blue-500 to-blue-600 text-white text-xs font-medium rounded-md hover:from-blue-600 hover:to-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 sm:w-auto w-full"
              >
                {isSubmitting ? (
                  <span className="flex items-center justify-center gap-1">
                    <svg className="animate-spin h-3 w-3" viewBox="0 0 24 24">
                      <circle
                        className="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        strokeWidth="4"
                        fill="none"
                      />
                      <path
                        className="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                      />
                    </svg>
                    Creating...
                  </span>
                ) : (
                  "Create Widget"
                )}
              </button>
            )}
          </form.Subscribe>
        </div>
      </form>
    </div>
  );
};

// Add these helper functions at the bottom of the file
function rgbToHex(r: number, g: number, b: number): string {
  return (
    "#" +
    [r, g, b]
      .map((x) => {
        const hex = Math.round(x).toString(16);
        return hex.length === 1 ? "0" + hex : hex;
      })
      .join("")
  );
}

function hexToRgb(hex: string): [number, number, number] {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? [
        parseInt(result[1], 16),
        parseInt(result[2], 16),
        parseInt(result[3], 16),
      ]
    : [0, 0, 0];
}
