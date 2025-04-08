import React from "react";

interface AutoFormProps<T extends object> {
  data: T;
  onChange: (updatedData: Partial<T>) => void; // Allow partial updates
}

// Basic component to automatically generate form fields based on data type
export default function AutoForm<T extends object>({
  data,
  onChange,
}: AutoFormProps<T>) {
  const handleChange = (key: keyof T, value: string | number | boolean) => {
    // Perform basic type coercion based on original type
    const originalType = typeof data[key];
    let coercedValue: string | number | boolean = value;

    if (originalType === "number") {
      coercedValue = Number(value) || 0; // Default to 0 if conversion fails
    } else if (originalType === "boolean") {
      // Checkbox passes boolean directly, but guard just in case
      coercedValue = Boolean(value);
    }

    onChange({ [key]: coercedValue } as Partial<T>);
  };

  // Simple function to format object keys into labels
  const formatLabel = (key: string): string => {
    return key
      .replace(/_/g, " ")
      .replace(/\b\w/g, (char) => char.toUpperCase()); // Capitalize words
  };

  return (
    <form className="space-y-3">
      {Object.entries(data).map(([key, value]) => {
        const inputId = `setting-${key}`;
        let inputElement: React.ReactNode;

        switch (typeof value) {
          case "boolean":
            inputElement = (
              <input
                type="checkbox"
                id={inputId}
                className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                checked={value}
                onChange={(e) => handleChange(key as keyof T, e.target.checked)}
              />
            );
            break;
          case "number":
            inputElement = (
              <input
                type="number"
                id={inputId}
                className="border rounded p-1 w-full text-sm"
                value={value}
                onChange={(e) => handleChange(key as keyof T, e.target.value)}
              />
            );
            break;
          case "string":
            // Could add select logic here if needed based on key or schema
            inputElement = (
              <input
                type="text"
                id={inputId}
                className="border rounded p-1 w-full text-sm"
                value={value}
                onChange={(e) => handleChange(key as keyof T, e.target.value)}
              />
            );
            break;
          default:
            inputElement = (
              <span className="text-xs text-gray-500">
                (Unsupported type: {typeof value})
              </span>
            );
        }

        // For boolean, render checkbox slightly differently (inline with label)
        if (typeof value === "boolean") {
          return (
            <div key={key} className="flex items-center space-x-2">
              {inputElement}
              <label htmlFor={inputId} className="text-sm font-medium">
                {formatLabel(key)}
              </label>
            </div>
          );
        }

        // Layout for other types
        return (
          <div key={key}>
            <label
              htmlFor={inputId}
              className="block text-xs font-medium mb-0.5"
            >
              {formatLabel(key)}
            </label>
            {inputElement}
          </div>
        );
      })}
    </form>
  );
}
