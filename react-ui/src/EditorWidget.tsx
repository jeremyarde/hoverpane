import { useState, useEffect } from "react";
import { ChevronDownIcon, ChevronRightIcon } from "@heroicons/react/24/outline";
import { Modifier, WidgetConfiguration, WidgetModifier } from "./types";

export default function EditWidget() {
  const [data, setData] = useState<WidgetConfiguration[]>([]);
  const [selectedWidget, setSelectedWidget] = useState<string | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [modifierType, setModifierType] = useState<"scrape" | "refresh" | null>(
    null
  );
  const [modifierConfig, setModifierConfig] = useState<Modifier>();
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
      const widgetDetails = data.map((item: WidgetConfiguration) => ({
        id: item.widget_id,
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

    // Convert the frontend format to the backend format
    const backendModifier: WidgetModifier = {
      id: 0, // This will be set by the backend
      widget_id: selectedWidget,
      modifier_type: modifierConfig as Modifier,
    };

    try {
      const response = await fetch(
        `http://127.0.0.1:3000/widgets/${selectedWidget}/modifiers`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(backendModifier),
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
      setModifierConfig(undefined);
    } catch (error) {
      console.error("Failed to save modifier:", error);
    }
  };

  const handleDeleteModifier = async (widgetId: string, modifierId: number) => {
    try {
      const response = await fetch(
        `http://127.0.0.1:3000/widgets/${widgetId}/modifiers/${modifierId}`,
        {
          method: "DELETE",
        }
      );

      if (!response.ok) throw new Error("Failed to delete modifier");

      // Refresh modifiers for this widget
      const modifiersResponse = await fetch(
        `http://127.0.0.1:3000/widgets/${widgetId}/modifiers`
      );
      const updatedModifiers = await modifiersResponse.json();
      setWidgetModifiers((prev) => ({
        ...prev,
        [widgetId]: updatedModifiers,
      }));
    } catch (error) {
      console.error("Failed to delete modifier:", error);
    }
  };

  const renderModifierDetails = (modifier: WidgetModifier) => {
    if (modifier.modifier_type.type === "scrape") {
      return (
        <div className="text-sm text-gray-600">
          Scrape: {modifier.modifier_type.content.selector}
        </div>
      );
    }
    if (modifier.modifier_type.type === "refresh") {
      return <div className="text-sm text-gray-600">Auto Refresh</div>;
    }
    return null;
  };

  return (
    <div className="p-4 h-full">
      {/* Widget List */}
      <div className="h-full">
        <h2 className="text-xl font-bold mb-4">Widget Modifiers</h2>
        <div className="h-[calc(100%-2rem)] overflow-auto">
          {data.length > 0 ? (
            data.map((item) => (
              <div
                key={item.widget_id}
                className="border border-gray-200 rounded-lg shadow-sm hover:shadow-md transition-shadow"
              >
                <div className="p-4">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => toggleWidget(item.widget_id)}
                        className="text-gray-500 hover:text-gray-700"
                      >
                        {expandedWidgets.has(item.widget_id) ? (
                          <ChevronDownIcon className="h-5 w-5" />
                        ) : (
                          <ChevronRightIcon className="h-5 w-5" />
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
                      onClick={() => handleAddModifier(item.widget_id)}
                      className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-md transition-colors"
                    >
                      Add Modifier
                    </button>
                  </div>
                </div>

                {/* Expanded Content */}
                {expandedWidgets.has(item.widget_id) && (
                  <div className="border-t border-gray-200 p-4 bg-gray-50">
                    <h4 className="font-medium mb-2">Active Modifiers</h4>
                    {widgetModifiers[item.widget_id]?.length > 0 ? (
                      <div className="space-y-2">
                        {widgetModifiers[item.widget_id].map((modifier) => (
                          <div
                            key={modifier.id}
                            className="bg-white p-3 rounded border border-gray-200"
                          >
                            {renderModifierDetails(modifier)}
                            <button
                              onClick={() =>
                                handleDeleteModifier(
                                  item.widget_id,
                                  modifier.id
                                )
                              }
                              className="ml-2 text-red-500 hover:text-red-700"
                            >
                              Delete
                            </button>
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
                  value={
                    modifierConfig?.type === "scrape"
                      ? modifierConfig?.content?.selector || ""
                      : ""
                  }
                  onChange={(e) =>
                    setModifierConfig({
                      ...modifierConfig,
                      type: "scrape",
                      content: {
                        selector: e.target.value,
                      },
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
                  value={
                    modifierConfig?.type === "refresh"
                      ? modifierConfig?.content?.interval_sec || ""
                      : ""
                  }
                  onChange={(e) =>
                    setModifierConfig({
                      ...modifierConfig,
                      type: "refresh",
                      content: {
                        interval_sec: Number(e.target.value),
                      },
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
