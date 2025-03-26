import { AnyFieldApi, formOptions, useForm } from "@tanstack/react-form";
import { type } from "arktype";

// example:
// const user = type({
// 	name: "string",
// 	platform: "'android' | 'ios'",
// 	"versions?": "(number | string)[]"
// })

const newWidget = type({
  title: "string | undefined",
  widget_type: "'file' | 'url' | 'source'",
  level: "'normal' | 'alwaysontop' | 'alwaysonbottom'",
  // optional fields
  refresh_interval: "number | undefined",
  url: "string | undefined",
  selector: "string | undefined",
  html: "string | undefined",
});

const formOpts = formOptions({
  defaultValues: {
    title: "",
    widget_type: "source",
    level: "normal",
    url: "",
    selector: "",
    refresh_interval: "240",
  },
  validators: {
    onChange: newWidget,
  },
});

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

export const NewWidgetForm = () => {
  const form = useForm({
    ...formOpts,
    onSubmit: async ({ value }) => {
      console.log(value);
    },
  });

  return (
    <div className="max-w-md mx-auto p-3 bg-white rounded-xl shadow-lg border border-gray-100">
      <form
        className="space-y-2"
        onSubmit={(e) => {
          e.preventDefault();
          e.stopPropagation();
          form.handleSubmit();
        }}
      >
        {/* Compact header with type selection */}
        <div className="flex items-center gap-2 mb-3">
          <form.Field name="widget_type">
            {(field) => (
              <div className="relative flex-shrink-0">
                <select
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)}
                  className="h-8 pr-8 pl-2 text-xs bg-gray-50 rounded-lg appearance-none focus:outline-none focus:ring-1 focus:ring-blue-500"
                >
                  <option value="source">📊</option>
                  <option value="file">📄</option>
                  <option value="url">🔗</option>
                  <option value="display">📺</option>
                  <option value="tracker">📍</option>
                  <option value="controls">🎮</option>
                </select>
                <div className="absolute right-2 top-2.5 text-gray-400 pointer-events-none text-[8px]">
                  ▼
                </div>
              </div>
            )}
          </form.Field>

          <form.Field
            name="title"
            validators={{
              onChange: ({ value }) =>
                !value
                  ? "A title is required"
                  : value.length < 3
                  ? "Title must be at least 3 characters"
                  : undefined,
              onChangeAsyncDebounceMs: 500,
              onChangeAsync: async ({ value }) => {
                await new Promise((resolve) => setTimeout(resolve, 1000));
                return value.includes("error") && 'No "error" allowed in title';
              },
            }}
          >
            {(field) => (
              <div className="relative flex-grow">
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
        </div>

        {/* Source-specific fields in a compact card */}
        {form.getFieldValue("widget_type") === "source" && (
          <div className="bg-gray-50 rounded-lg p-2 space-y-2">
            <form.Field name="url">
              {(field) => (
                <div className="relative">
                  <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                    🔗
                  </div>
                  <input
                    type="url"
                    placeholder="URL"
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
                    placeholder="CSS Selector"
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
        <div className="flex items-center gap-2 pt-2 border-t border-gray-100">
          <form.Field name="refresh_interval">
            {(field) => (
              <div className="relative flex-shrink-0">
                <div className="absolute left-2 top-1.5 text-gray-400 text-xs">
                  ⏱️
                </div>
                <input
                  type="number"
                  min="0"
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)}
                  className="w-20 h-7 pl-7 pr-2 text-xs bg-gray-50 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500"
                />
                <FieldInfo field={field} />
              </div>
            )}
          </form.Field>

          <form.Field name="level">
            {(field) => (
              <div className="relative flex-shrink-0">
                <select
                  value={field.state.value}
                  onChange={(e) => field.handleChange(e.target.value)}
                  className="h-7 px-2 text-xs bg-gray-50 rounded-md appearance-none focus:outline-none focus:ring-1 focus:ring-blue-500"
                >
                  <option value="normal">⚫</option>
                  <option value="alwaysontop">⬆️</option>
                  <option value="alwaysonbottom">⬇️</option>
                </select>
              </div>
            )}
          </form.Field>

          <form.Subscribe
            selector={(state) => [state.canSubmit, state.isSubmitting]}
          >
            {([canSubmit, isSubmitting]) => (
              <button
                type="submit"
                disabled={!canSubmit}
                className="flex-grow h-7 bg-gradient-to-r from-blue-500 to-blue-600 text-white text-xs font-medium rounded-md hover:from-blue-600 hover:to-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200"
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

          <button
            type="button"
            onClick={() => form.reset()}
            className="flex-shrink-0 w-7 h-7 flex items-center justify-center text-gray-400 hover:text-gray-600 bg-gray-50 rounded-md focus:outline-none focus:ring-1 focus:ring-gray-400 transition-colors duration-200"
          >
            ↺
          </button>
        </div>
      </form>
    </div>
  );
};
