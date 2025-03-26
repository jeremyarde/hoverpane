"use client";

import { useState, useEffect } from "react";
import { z } from "zod";
// import { zodResolver } from "@hookform/resolvers/zod"
// import { useForm } from "react-hook-form"
import { useForm, FieldApi } from "@tanstack/react-form";

// Define the schema with Zod
export const widgetSchema = z
  .object({
    title: z.string().optional(),
    widget_type: z.enum(["source", "url", "file"]),
    level: z.enum(["normal", "alwaysontop", "alwaysonbottom"]),
    url: z.string().transform((val, ctx) => {
      if (!val) return val;
      try {
        new URL(val);
        return val;
      } catch {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message: "Invalid URL format",
        });
        return val;
      }
    }),
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
  })
  .refine(
    (data) => {
      if (data.widget_type === "file") return true;
      return !!data.url;
    },
    {
      message: "URL is required for this widget type",
      path: ["url"],
    }
  );

type WidgetFormValues = z.infer<typeof widgetSchema>;

const defaultValues: WidgetFormValues = {
  widget_type: "source",
  level: "normal",
  position: [0, 0],
  size: [300, 200],
  background_color: [245, 246, 247, 1],
  resizeable: true,
  movable: true,
  url: "",
  title: "",
  selector: "",
  html: "",
  refresh_interval: 0,
};

interface FieldErrorProps {
  field: FieldApi<any, any>;
}

function FieldError({ field }: FieldErrorProps) {
  const errors = field.state.meta.touchedErrors || field.state.meta.errors;
  return (
    <>
      {errors?.length ? (
        <em className="text-red-500 text-sm">{errors.join(", ")}</em>
      ) : null}
    </>
  );
}

interface WidgetFormProps {
  onBackgroundColorChange: (color: [number, number, number, number]) => void;
}

export default function WidgetForm({
  onBackgroundColorChange,
}: WidgetFormProps) {
  const [activeTab, setActiveTab] = useState("layout");

  const form = useForm({
    defaultValues,
    onSubmit: async ({ value }) => {
      console.log("Submitting:", value);
      await new Promise((resolve) => setTimeout(resolve, 1000));
    },
  });

  return (
    <div className="px-6 py-4">
      <form.Subscribe
        selector={(state) => state.values.background_color}
        children={(background_color) => {
          if (background_color) {
            onBackgroundColorChange(background_color);
          }
          return null;
        }}
      />
      <form
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
        className="space-y-6"
      >
        {/* Widget Type Selection */}
        <div className="grid grid-cols-2 gap-4">
          <form.Field
            name="widget_type"
            children={(field) => (
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Widget Type
                </label>
                <select
                  className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  value={field.state.value}
                  onChange={(e) =>
                    field.handleChange(
                      e.target.value as WidgetFormValues["widget_type"]
                    )
                  }
                >
                  <option value="source">Source</option>
                  <option value="url">URL</option>
                  <option value="file">File</option>
                </select>
                <FieldError field={field} />
              </div>
            )}
          />

          <form.Field
            name="title"
            children={(field) => (
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Title
                </label>
                <input
                  type="text"
                  className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  value={field.state.value ?? ""}
                  onChange={(e) => field.handleChange(e.target.value)}
                  placeholder="Widget title"
                />
                <FieldError field={field} />
              </div>
            )}
          />
        </div>

        {/* URL and Selector Fields */}
        <form.Field
          name="url"
          children={(field) => (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                URL
              </label>
              <input
                type="text"
                className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                value={field.state.value ?? ""}
                onChange={(e) => field.handleChange(e.target.value)}
                placeholder="https://example.com"
              />
              <FieldError field={field} />
            </div>
          )}
        />

        <form.Field
          name="selector"
          children={(field) => (
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Selector
              </label>
              <input
                type="text"
                className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                value={field.state.value ?? ""}
                onChange={(e) => field.handleChange(e.target.value)}
                placeholder="CSS selector"
              />
              <FieldError field={field} />
            </div>
          )}
        />

        {/* Tabs */}
        <div className="border-b border-gray-200">
          <nav className="-mb-px flex space-x-8" aria-label="Tabs">
            {["layout", "behavior", "appearance"].map((tab) => (
              <button
                key={tab}
                type="button"
                className={`${
                  activeTab === tab
                    ? "border-blue-500 text-blue-600"
                    : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300"
                } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors duration-150`}
                onClick={() => setActiveTab(tab)}
                aria-current={activeTab === tab ? "page" : undefined}
              >
                {tab.charAt(0).toUpperCase() + tab.slice(1)}
              </button>
            ))}
          </nav>
        </div>

        {/* Tab Content */}
        <div className="mt-6">
          {activeTab === "layout" && (
            <div className="grid grid-cols-2 gap-6">
              <form.Field
                name="position"
                children={(field) => (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Position [X, Y]
                    </label>
                    <div className="flex space-x-2">
                      <input
                        type="number"
                        className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[0] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            Number(e.target.value),
                            field.state.value?.[1] ?? 0,
                          ])
                        }
                        min="0"
                        placeholder="X"
                      />
                      <input
                        type="number"
                        className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[1] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            field.state.value?.[0] ?? 0,
                            Number(e.target.value),
                          ])
                        }
                        min="0"
                        placeholder="Y"
                      />
                    </div>
                    <FieldError field={field} />
                  </div>
                )}
              />

              <form.Field
                name="size"
                children={(field) => (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Size [Width, Height]
                    </label>
                    <div className="flex space-x-2">
                      <input
                        type="number"
                        className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[0] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            Number(e.target.value),
                            field.state.value?.[1] ?? 0,
                          ])
                        }
                        min="0"
                        placeholder="Width"
                      />
                      <input
                        type="number"
                        className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[1] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            field.state.value?.[0] ?? 0,
                            Number(e.target.value),
                          ])
                        }
                        min="0"
                        placeholder="Height"
                      />
                    </div>
                    <FieldError field={field} />
                  </div>
                )}
              />
            </div>
          )}

          {activeTab === "behavior" && (
            <div className="grid grid-cols-2 gap-6">
              <form.Field
                name="level"
                children={(field) => (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Window Level
                    </label>
                    <select
                      className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                      value={field.state.value}
                      onChange={(e) =>
                        field.handleChange(
                          e.target.value as WidgetFormValues["level"]
                        )
                      }
                    >
                      <option value="normal">Normal</option>
                      <option value="alwaysontop">Always on Top</option>
                      <option value="alwaysonbottom">Always on Bottom</option>
                    </select>
                    <FieldError field={field} />
                  </div>
                )}
              />

              <form.Field
                name="refresh_interval"
                children={(field) => (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Refresh Interval (ms)
                    </label>
                    <input
                      type="number"
                      className="w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                      value={field.state.value ?? ""}
                      onChange={(e) =>
                        field.handleChange(
                          e.target.value ? Number(e.target.value) : undefined
                        )
                      }
                      min="0"
                      placeholder="e.g. 1000"
                    />
                    <FieldError field={field} />
                  </div>
                )}
              />

              <div className="col-span-2 grid grid-cols-2 gap-6">
                <form.Field
                  name="resizeable"
                  children={(field) => (
                    <div className="flex items-center space-x-3">
                      <input
                        type="checkbox"
                        className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked={field.state.value ?? false}
                        onChange={(e) => field.handleChange(e.target.checked)}
                      />
                      <label className="text-sm font-medium text-gray-700">
                        Resizeable
                      </label>
                      <FieldError field={field} />
                    </div>
                  )}
                />

                <form.Field
                  name="movable"
                  children={(field) => (
                    <div className="flex items-center space-x-3">
                      <input
                        type="checkbox"
                        className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                        checked={field.state.value ?? false}
                        onChange={(e) => field.handleChange(e.target.checked)}
                      />
                      <label className="text-sm font-medium text-gray-700">
                        Movable
                      </label>
                      <FieldError field={field} />
                    </div>
                  )}
                />
              </div>
            </div>
          )}

          {activeTab === "appearance" && (
            <div>
              <form.Field
                name="background_color"
                children={(field) => (
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Background Color
                    </label>
                    <div className="grid grid-cols-4 gap-3">
                      <input
                        type="number"
                        className="p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[0] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            Number(e.target.value),
                            field.state.value?.[1] ?? 0,
                            field.state.value?.[2] ?? 0,
                            field.state.value?.[3] ?? 1,
                          ])
                        }
                        min="0"
                        max="255"
                        placeholder="R"
                      />
                      <input
                        type="number"
                        className="p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[1] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            field.state.value?.[0] ?? 0,
                            Number(e.target.value),
                            field.state.value?.[2] ?? 0,
                            field.state.value?.[3] ?? 1,
                          ])
                        }
                        min="0"
                        max="255"
                        placeholder="G"
                      />
                      <input
                        type="number"
                        className="p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[2] ?? 0}
                        onChange={(e) =>
                          field.handleChange([
                            field.state.value?.[0] ?? 0,
                            field.state.value?.[1] ?? 0,
                            Number(e.target.value),
                            field.state.value?.[3] ?? 1,
                          ])
                        }
                        min="0"
                        max="255"
                        placeholder="B"
                      />
                      <input
                        type="number"
                        className="p-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                        value={field.state.value?.[3] ?? 1}
                        onChange={(e) =>
                          field.handleChange([
                            field.state.value?.[0] ?? 0,
                            field.state.value?.[1] ?? 0,
                            field.state.value?.[2] ?? 0,
                            Number(e.target.value),
                          ])
                        }
                        min="0"
                        max="1"
                        step="0.1"
                        placeholder="A"
                      />
                    </div>
                    <div
                      className="mt-3 h-10 rounded-md border border-gray-200 shadow-inner"
                      style={{
                        backgroundColor: `rgba(${
                          field.state.value?.join(",") ?? "245,246,247,1"
                        })`,
                      }}
                    />
                    <FieldError field={field} />
                  </div>
                )}
              />
            </div>
          )}
        </div>

        <div className="flex justify-end space-x-3 pt-6 border-t border-gray-100">
          <button
            type="button"
            className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            onClick={() => form.reset()}
          >
            Reset
          </button>
          <form.Subscribe
            selector={(state) => [state.canSubmit, state.isSubmitting]}
            children={([canSubmit, isSubmitting]) => (
              <button
                type="submit"
                disabled={!canSubmit}
                className={`px-4 py-2 text-sm font-medium text-white rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-offset-2 ${
                  canSubmit
                    ? "bg-blue-600 hover:bg-blue-700 focus:ring-blue-500"
                    : "bg-blue-400 cursor-not-allowed"
                }`}
              >
                {isSubmitting ? "Creating..." : "Create Widget"}
              </button>
            )}
          />
        </div>
      </form>
    </div>
  );
}
