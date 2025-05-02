import { useState, useEffect } from "react";
import {
  ChevronDownIcon,
  ChevronRightIcon,
  TrashIcon,
  EyeIcon,
  ArrowsPointingOutIcon,
  PlusIcon,
  ClockIcon,
  CodeBracketIcon,
} from "@heroicons/react/24/outline";
import {
  ApiAction,
  Modifier,
  WidgetBounds,
  WidgetConfiguration,
  WidgetModifier,
} from "./types";

export default function EditWidgets() {
  const [widgetConfigs, setWidgetConfigs] = useState<WidgetConfiguration[]>([]);
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
      const response = await fetch(`http://127.0.0.1:3111/widgets`, {});
      const data = await response.json();
      const widgetDetails = data.map((item: WidgetConfiguration) => ({
        ...item,
      }));
      console.log("data from fetch", widgetDetails);
      setWidgetConfigs(widgetDetails);
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
            `http://127.0.0.1:3111/widgets/${widgetId}/modifiers`
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
      id: 0,
      widget_id: selectedWidget,
      modifier_type: modifierConfig as Modifier,
    };

    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${selectedWidget}/modifiers`,
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
        `http://127.0.0.1:3111/widgets/${selectedWidget}/modifiers`
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

  const handleDeleteModifier = async (widgetId: string, modifierId: string) => {
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widgetId}/modifiers/${modifierId}`,
        {
          method: "DELETE",
        }
      );

      if (!response.ok) throw new Error("Failed to delete modifier");

      // Refresh modifiers for this widget
      const modifiersResponse = await fetch(
        `http://127.0.0.1:3111/widgets/${widgetId}/modifiers`
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

  const handleDeleteWidget = async (widget_id: string) => {
    console.log("Deleting widget", widget_id);
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widget_id}`,
        {
          method: "DELETE",
        }
      );
      if (!response.ok) throw new Error("Failed to delete widget");

      // Remove the widget from the local state
      setWidgetConfigs((prev) => prev.filter((w) => w.widget_id !== widget_id));

      // Remove any associated modifiers from state
      setWidgetModifiers((prev) => {
        const newState = { ...prev };
        delete newState[widget_id];
        return newState;
      });

      // Remove from expanded widgets if it was expanded
      setExpandedWidgets((prev) => {
        const newSet = new Set(prev);
        newSet.delete(widget_id);
        return newSet;
      });
    } catch (error) {
      console.error("Failed to delete widget:", error);
    }
  };

  const handleHideWidget = async (widget_id: string) => {
    console.log("Hiding widget", widget_id);
    const hideAction: ApiAction = {
      type: "togglewidgetvisibility",
      content: {
        widget_id: widget_id,
        visible: false,
      },
    };
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widget_id}`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(hideAction),
        }
      );
      if (!response.ok) throw new Error("Failed to hide widget");
    } catch (error) {
      console.error("Failed to hide widget:", error);
    }
  };

  const handleUpdateWidgetBounds = async (
    widget_id: string,
    bounds: WidgetBounds
  ) => {
    console.log("Updating widget bounds", widget_id, bounds);
    const updateAction: ApiAction = {
      type: "updatewidgetbounds",
      content: {
        widget_id: widget_id,
        bounds: bounds,
      },
    };
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widget_id}`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(updateAction),
        }
      );
      if (!response.ok) throw new Error("Failed to update widget bounds");
    } catch (error) {
      console.error("Failed to update widget bounds:", error);
    }
  };

  return (
    <div className="p-6 h-full bg-gray-50">
      <div className="max-w-4xl mx-auto">
        <div className="flex items-center justify-between mb-6">
          <h1 className="text-2xl font-bold text-gray-900">Widgets</h1>
          <div className="text-sm text-gray-500">
            {widgetConfigs.length} widget{widgetConfigs.length !== 1 ? "s" : ""}
          </div>
        </div>

        <div className="space-y-3">
          {widgetConfigs.length > 0 ? (
            widgetConfigs.map((widget) => (
              <div
                key={widget.widget_id}
                className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden"
              >
                {/* Widget Header */}
                <div className="p-4">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-4">
                      <button
                        onClick={() => toggleWidget(widget.widget_id)}
                        className="p-1.5 rounded-full hover:bg-gray-100 transition-colors"
                      >
                        {expandedWidgets.has(widget.widget_id) ? (
                          <ChevronDownIcon className="h-5 w-5 text-gray-500" />
                        ) : (
                          <ChevronRightIcon className="h-5 w-5 text-gray-500" />
                        )}
                      </button>
                      <div>
                        <h3 className="font-medium text-gray-900">
                          {widget.title}
                        </h3>
                        <p className="text-sm text-gray-500">
                          Level: {widget.level}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => handleAddModifier(widget.widget_id)}
                        className="p-2 text-blue-600 hover:text-blue-800 transition-colors"
                        title="Add Modifier"
                      >
                        <PlusIcon className="h-5 w-5" />
                      </button>
                      <button
                        onClick={() => handleHideWidget(widget.widget_id)}
                        className="p-2 text-gray-600 hover:text-gray-800 transition-colors"
                        title="Hide Widget"
                      >
                        <EyeIcon className="h-5 w-5" />
                      </button>
                      <button
                        onClick={() =>
                          handleUpdateWidgetBounds(widget.widget_id, {
                            x: 100,
                            y: 100,
                            width: 800,
                            height: 600,
                          })
                        }
                        className="p-2 text-gray-600 hover:text-gray-800 transition-colors"
                        title="Update Bounds"
                      >
                        <ArrowsPointingOutIcon className="h-5 w-5" />
                      </button>
                      <button
                        onClick={() => handleDeleteWidget(widget.widget_id)}
                        className="p-2 text-red-600 hover:text-red-800 transition-colors"
                        title="Delete Widget"
                      >
                        <TrashIcon className="h-5 w-5" />
                      </button>
                    </div>
                  </div>
                </div>

                {/* Modifiers Section */}
                {expandedWidgets.has(widget.widget_id) && (
                  <div className="border-t border-gray-200">
                    <div className="p-4 bg-gray-50">
                      <div className="flex items-center justify-between mb-3">
                        <h4 className="text-sm font-medium text-gray-700">
                          Modifiers
                        </h4>
                        <button
                          onClick={() => handleAddModifier(widget.widget_id)}
                          className="flex items-center gap-1.5 text-sm text-blue-600 hover:text-blue-800 transition-colors"
                        >
                          <PlusIcon className="h-4 w-4" />
                          <span>Add Modifier</span>
                        </button>
                      </div>

                      {widgetModifiers[widget.widget_id]?.length > 0 ? (
                        <div className="space-y-2">
                          {widgetModifiers[widget.widget_id].map((modifier) => (
                            <div
                              key={modifier.id}
                              className="bg-white p-3 rounded border border-gray-200 flex items-center justify-between group hover:border-gray-300 transition-colors"
                            >
                              <div className="flex items-center gap-3">
                                {modifier.modifier_type.type === "scrape" ? (
                                  <div className="p-1.5 bg-blue-50 rounded-full">
                                    <CodeBracketIcon className="h-4 w-4 text-blue-600" />
                                  </div>
                                ) : (
                                  <div className="p-1.5 bg-purple-50 rounded-full">
                                    <ClockIcon className="h-4 w-4 text-purple-600" />
                                  </div>
                                )}
                                <div className="flex-1">
                                  {modifier.modifier_type.type === "scrape" ? (
                                    <div className="text-sm text-gray-900">
                                      Scrape:{" "}
                                      {modifier.modifier_type.content.selector}
                                    </div>
                                  ) : (
                                    <div className="text-sm text-gray-900">
                                      Auto Refresh
                                    </div>
                                  )}
                                </div>
                              </div>
                              <button
                                onClick={() =>
                                  handleDeleteModifier(
                                    widget.widget_id,
                                    modifier.id.toString()
                                  )
                                }
                                className="p-1.5 text-gray-400 hover:text-red-600 transition-colors opacity-0 group-hover:opacity-100"
                                title="Delete Modifier"
                              >
                                <TrashIcon className="h-4 w-4" />
                              </button>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-center py-6 bg-white rounded border border-gray-200">
                          <p className="text-sm text-gray-500 mb-2">
                            No modifiers added yet
                          </p>
                          <button
                            onClick={() => handleAddModifier(widget.widget_id)}
                            className="text-sm text-blue-600 hover:text-blue-800 transition-colors"
                          >
                            Add your first modifier
                          </button>
                        </div>
                      )}
                    </div>
                  </div>
                )}
              </div>
            ))
          ) : (
            <div className="text-center py-12 text-gray-500">
              <p className="text-lg mb-2">No widgets found</p>
              <p className="text-sm">Create a new widget to get started</p>
            </div>
          )}
        </div>
      </div>

      {/* Modifier Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-8 rounded-lg w-[32rem] max-w-[90vw] shadow-xl">
            <h3 className="text-xl font-semibold mb-6">Add Modifier</h3>

            {/* Modifier Type Selection */}
            <div className="mb-6">
              <label className="block text-sm font-medium mb-2 text-gray-700">
                Modifier Type
              </label>
              <select
                className="w-full border border-gray-300 rounded-md p-2.5 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
              <div className="mb-6">
                <label className="block text-sm font-medium mb-2 text-gray-700">
                  CSS Selector
                </label>
                <input
                  type="text"
                  className="w-full border border-gray-300 rounded-md p-2.5 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
                        modifier_id: modifierConfig?.content?.modifier_id || "",
                        selector: e.target.value,
                      },
                    })
                  }
                />
              </div>
            )}

            {modifierType === "refresh" && (
              <div className="mb-6">
                <label className="block text-sm font-medium mb-2 text-gray-700">
                  Refresh Interval (seconds)
                </label>
                <input
                  type="number"
                  className="w-full border border-gray-300 rounded-md p-2.5 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
                        modifier_id: modifierConfig?.content?.modifier_id || "",
                        interval_sec: Number(e.target.value),
                      },
                    })
                  }
                />
              </div>
            )}

            {/* Modal Actions */}
            <div className="flex justify-end space-x-3">
              <button
                onClick={() => setIsModalOpen(false)}
                className="px-4 py-2 text-gray-600 hover:text-gray-800 transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleSaveModifier}
                className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors"
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
