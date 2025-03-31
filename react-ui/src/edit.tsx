import { useState, useEffect } from "react";
import { ChevronDownIcon, ChevronUpIcon } from "@heroicons/react/24/outline";

interface WidgetDetails {
  id: string;
  title: string;
  widget_type: {
    url?: { url: string };
    file?: { html: string };
  };
  level: "alwaysontop" | "normal" | "alwaysonbottom";
}

interface WidgetModifier {
  id: string;
  widget_id: string;
  modifier_type: {
    scrape?: { selector: string };
    refresh?: { interval: number };
  };
}

interface ModifierConfig {
  type: "scrape" | "refresh";
  config: {
    selector?: string;
    interval?: number;
  };
}

export default function EditWidget() {
  const [data, setData] = useState<WidgetDetails[]>([]);
  const [selectedWidget, setSelectedWidget] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [modifierType, setModifierType] = useState<"scrape" | "refresh" | null>(
    null
  );
  const [modifierConfig, setModifierConfig] = useState<
    ModifierConfig["config"]
  >({});
  const [expandedWidgets, setExpandedWidgets] = useState<Set<string>>(
    new Set()
  );
  const [widgetModifiers, setWidgetModifiers] = useState<
    Record<string, WidgetModifier[]>
  >({});

  useEffect(() => {
    const fetchData = async () => {
      const response = await fetch("http://127.0.0.1:3000/widgets", {});
      const data = await response.json();
      const widgetDetails = data.map((item: WidgetDetails) => ({
        id: item.id,
        title: item.title,
        widget_type: item.widget_type,
        level: item.level,
      }));
      console.log("data from fetch", widgetDetails);
      setData(widgetDetails);
    };
    fetchData();
  }, []);

  const toggleWidget = async (widgetId: string) => {
    const newExpandedWidgets = new Set(expandedWidgets);
    if (expandedWidgets.has(widgetId)) {
      newExpandedWidgets.delete(widgetId);
    } else {
      newExpandedWidgets.add(widgetId);
      // Fetch modifiers if not already loaded
      if (!widgetModifiers[widgetId]) {
        try {
          const response = await fetch(
            `http://127.0.0.1:3000/widgets/${widgetId}/modifiers`
          );
          if (!response.ok) throw new Error("Failed to fetch modifiers");
          const modifiers = await response.json();
          setWidgetModifiers((prev) => ({
            ...prev,
            [widgetId]: modifiers,
          }));
        } catch (error) {
          console.error("Failed to fetch modifiers:", error);
        }
      }
    }
    setExpandedWidgets(newExpandedWidgets);
  };

  const handleAddModifier = (widgetId: string) => {
    setSelectedWidget(widgetId);
    setIsModalOpen(true);
  };

  const handleSaveModifier = async () => {
    if (!selectedWidget || !modifierType) return;

    const modifier: ModifierConfig = {
      type: modifierType,
      config: modifierConfig,
    };

    try {
      const response = await fetch(
        `http://127.0.0.1:3000/widgets/${selectedWidget}/modifiers`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(modifier),
        }
      );

      if (!response.ok) throw new Error("Failed to save modifier");

      // Refresh modifiers for this widget
      const modifiersResponse = await fetch(
        `http://127.0.0.1:3000/widgets/${selectedWidget}/modifiers`
      );
      const updatedModifiers = await modifiersResponse.json();
      setWidgetModifiers((prev) => ({
        ...prev,
        [selectedWidget]: updatedModifiers,
      }));

      // Reset state
      setIsModalOpen(false);
      setSelectedWidget(null);
      setModifierType(null);
      setModifierConfig({});
    } catch (error) {
      console.error("Failed to save modifier:", error);
    }
  };

  const renderModifierDetails = (modifier: WidgetModifier) => {
    if (modifier.modifier_type.scrape) {
      return (
        <div className="text-sm text-gray-600">
          Scrape: {modifier.modifier_type.scrape.selector}
        </div>
      );
    }
    if (modifier.modifier_type.refresh) {
      return <div className="text-sm text-gray-600">Auto Refresh</div>;
    }
    return null;
  };

  return (
    <div className="p-4">
      {/* Widget List */}
      <div className="max-w-2xl mx-auto space-y-4">
        <h2 className="text-xl font-bold mb-4">Widget Modifiers</h2>
        {data.length > 0 ? (
          data.map((item) => (
            <div
              key={item.id}
              className="border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow"
            >
              <div className="p-4">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => toggleWidget(item.id)}
                      className="text-gray-500 hover:text-gray-700"
                    >
                      {expandedWidgets.has(item.id) ? (
                        <ChevronUpIcon className="h-5 w-5" />
                      ) : (
                        <ChevronDownIcon className="h-5 w-5" />
                      )}
                    </button>
                    <div>
                      <h3 className="font-semibold">{item.title}</h3>
                      <p className="text-sm text-gray-600">
                        Level: {item.level}
                      </p>
                    </div>
                  </div>
                  <button
                    onClick={() => handleAddModifier(item.id)}
                    className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-colors"
                  >
                    Add Modifier
                  </button>
                </div>
              </div>

              {/* Expanded Content */}
              {expandedWidgets.has(item.id) && (
                <div className="border-t border-gray-200 p-4 bg-gray-50">
                  <h4 className="font-medium mb-2">Active Modifiers</h4>
                  {widgetModifiers[item.id]?.length > 0 ? (
                    <div className="space-y-2">
                      {widgetModifiers[item.id].map((modifier) => (
                        <div
                          key={modifier.id}
                          className="bg-white p-3 rounded border border-gray-200"
                        >
                          {renderModifierDetails(modifier)}
                        </div>
                      ))}
                    </div>
                  ) : (
                    <p className="text-sm text-gray-500">
                      No modifiers added yet
                    </p>
                  )}
                </div>
              )}
            </div>
          ))
        ) : (
          <div className="text-center text-gray-500">No widgets found</div>
        )}
      </div>

      {/* Modifier Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
          <div className="bg-white p-6 rounded-lg w-96">
            <h3 className="text-lg font-semibold mb-4">Add Modifier</h3>

            {/* Modifier Type Selection */}
            <div className="mb-4">
              <label className="block text-sm font-medium mb-2">
                Modifier Type
              </label>
              <select
                className="w-full border rounded-md p-2"
                value={modifierType || ""}
                onChange={(e) =>
                  setModifierType(e.target.value as "scrape" | "refresh" | null)
                }
              >
                <option value="">Select a type...</option>
                <option value="scrape">Scrape Element</option>
                <option value="refresh">Auto Refresh</option>
              </select>
            </div>

            {/* Modifier Configuration */}
            {modifierType === "scrape" && (
              <div className="mb-4">
                <label className="block text-sm font-medium mb-2">
                  CSS Selector
                </label>
                <input
                  type="text"
                  className="w-full border rounded-md p-2"
                  placeholder=".my-element"
                  value={modifierConfig.selector || ""}
                  onChange={(e) =>
                    setModifierConfig({
                      ...modifierConfig,
                      selector: e.target.value,
                    })
                  }
                />
              </div>
            )}

            {modifierType === "refresh" && (
              <div className="mb-4">
                <label className="block text-sm font-medium mb-2">
                  Refresh Interval (seconds)
                </label>
                <input
                  type="number"
                  className="w-full border rounded-md p-2"
                  min="1"
                  value={modifierConfig.interval || ""}
                  onChange={(e) =>
                    setModifierConfig({
                      ...modifierConfig,
                      interval: Number(e.target.value),
                    })
                  }
                />
              </div>
            )}

            {/* Modal Actions */}
            <div className="flex justify-end space-x-2">
              <button
                onClick={() => setIsModalOpen(false)}
                className="px-4 py-2 text-gray-600 hover:text-gray-800"
              >
                Cancel
              </button>
              <button
                onClick={handleSaveModifier}
                className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600"
                disabled={!modifierType}
              >
                Save
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
