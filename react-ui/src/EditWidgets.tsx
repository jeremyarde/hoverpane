import { useState, useEffect } from "react";
import {
  ChevronDownIcon,
  ChevronRightIcon,
  TrashIcon,
  EyeIcon,
  EyeSlashIcon,
  ArrowsPointingOutIcon,
  PlusIcon,
  ClockIcon,
  CodeBracketIcon,
} from "@heroicons/react/24/outline";
import {
  Modifier,
  WidgetBounds,
  WidgetConfiguration,
  WidgetModifier,
} from "./types";
import {
  addWidgetModifier,
  getWidgetModifiers,
  getWidgets,
} from "./clientInterface";
import Toast from "./Toast";

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
  const [editingBounds, setEditingBounds] = useState<{
    widgetId: string;
    bounds: WidgetBounds;
  } | null>(null);
  const [toast, setToast] = useState<{
    message: string;
    type: "success" | "error" | "info";
  } | null>(null);

  useEffect(() => {
    const fetchWidgets = async () => {
      try {
        const response = await getWidgets();
        if (!response.ok) throw new Error("Failed to fetch widgets");
        const data = await response.json();
        setWidgetConfigs(data);
      } catch (error) {
        console.error("Error fetching widgets:", error);
      }
    };

    fetchWidgets();
  }, []);

  useEffect(() => {
    const fetchModifiers = async () => {
      if (!selectedWidget) return;

      try {
        const response = await getWidgetModifiers(selectedWidget);
        if (!response.ok) throw new Error("Failed to fetch modifiers");
        const data = await response.json();
        setWidgetModifiers((prev) => ({
          ...prev,
          [selectedWidget]: data,
        }));
      } catch (error) {
        console.error("Error fetching modifiers:", error);
      }
    };

    fetchModifiers();
  }, [selectedWidget]);

  const toggleWidget = async (widgetId: string) => {
    const newExpandedWidgets = new Set(expandedWidgets);
    if (expandedWidgets.has(widgetId)) {
      newExpandedWidgets.delete(widgetId);
    } else {
      newExpandedWidgets.add(widgetId);
      setSelectedWidget(widgetId);
    }
    setExpandedWidgets(newExpandedWidgets);
  };

  const handleAddModifier = (widgetId: string) => {
    setSelectedWidget(widgetId);
    setModifierType(null);
    setModifierConfig(undefined);
    setIsModalOpen(true);
  };

  const handleSaveModifier = async () => {
    if (!selectedWidget || !modifierType || !modifierConfig) return;

    try {
      const response = await addWidgetModifier(selectedWidget, {
        id: 0,
        widget_id: selectedWidget,
        modifier_type: modifierConfig,
      });

      if (!response.ok) throw new Error("Failed to add modifier");

      const data = await response.json();
      setWidgetModifiers((prev) => ({
        ...prev,
        [selectedWidget]: [...(prev[selectedWidget] || []), data],
      }));
      setIsModalOpen(false);
      setToast({
        message: "Modifier added successfully",
        type: "success",
      });
    } catch (error) {
      console.error("Error adding modifier:", error);
      setToast({
        message: "Failed to add modifier",
        type: "error",
      });
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

      setWidgetModifiers((prev) => ({
        ...prev,
        [widgetId]: prev[widgetId].filter(
          (m) => m.id.toString() !== modifierId
        ),
      }));
    } catch (error) {
      console.error("Error deleting modifier:", error);
    }
  };

  const handleUpdateWidgetBounds = async (
    widgetId: string,
    bounds: WidgetBounds
  ) => {
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widgetId}`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            type: "updatewidgetbounds",
            content: {
              widget_id: widgetId,
              bounds: bounds,
            },
          }),
        }
      );

      if (!response.ok) throw new Error("Failed to update widget bounds");
    } catch (error) {
      console.error("Error updating widget bounds:", error);
    }
  };

  const handleDeleteWidget = async (widgetId: string) => {
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widgetId}`,
        {
          method: "DELETE",
        }
      );

      if (!response.ok) throw new Error("Failed to delete widget");

      setWidgetConfigs((prev) => prev.filter((w) => w.widget_id !== widgetId));
      setSelectedWidget(null);
    } catch (error) {
      console.error("Error deleting widget:", error);
    }
  };

  const handleHideWidget = async (widgetId: string) => {
    try {
      const response = await fetch(
        `http://127.0.0.1:3111/widgets/${widgetId}`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            type: "togglewidgetvisibility",
            content: {
              widget_id: widgetId,
              visible: false,
            },
          }),
        }
      );

      if (!response.ok) throw new Error("Failed to hide widget");

      setWidgetConfigs((prev) =>
        prev.map((w) =>
          w.widget_id === widgetId ? { ...w, visible: false } : w
        )
      );
    } catch (error) {
      console.error("Error hiding widget:", error);
    }
  };

  const handleBoundsChange = (field: keyof WidgetBounds, value: number) => {
    if (!editingBounds) return;
    setEditingBounds({
      ...editingBounds,
      bounds: {
        ...editingBounds.bounds,
        [field]: value,
      },
    });
  };

  const handleSaveBounds = async () => {
    if (!editingBounds) return;
    await handleUpdateWidgetBounds(
      editingBounds.widgetId,
      editingBounds.bounds
    );
    setEditingBounds(null);
  };

  return (
    <div className="h-full bg-gradient-to-br from-gray-50 to-gray-100">
      <div className="max-w-7xl mx-auto p-4">
        <div className="flex items-center justify-between mb-4">
          <div>
            <h1 className="text-2xl font-bold text-gray-900">Edit Widgets</h1>
            <p className="text-xs text-gray-500">
              {widgetConfigs.length} widget
              {widgetConfigs.length !== 1 ? "s" : ""} in your workspace
            </p>
          </div>
        </div>

        <div className="space-y-2">
          {widgetConfigs.length > 0 ? (
            widgetConfigs.map((widget) => (
              <div
                key={widget.widget_id}
                className="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden"
              >
                {/* Widget Header */}
                <div className="p-3">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => toggleWidget(widget.widget_id)}
                        className="p-1 rounded hover:bg-gray-100 transition-colors"
                      >
                        {expandedWidgets.has(widget.widget_id) ? (
                          <ChevronDownIcon className="h-4 w-4 text-gray-500" />
                        ) : (
                          <ChevronRightIcon className="h-4 w-4 text-gray-500" />
                        )}
                      </button>
                      <div>
                        <h3 className="text-sm font-semibold text-gray-900">
                          {widget.title}
                        </h3>
                        <p className="text-xs text-gray-500">
                          Level: {widget.level}
                        </p>
                      </div>
                    </div>
                    <div className="flex items-center gap-1">
                      <button
                        onClick={() =>
                          handleUpdateWidgetBounds(widget.widget_id, {
                            x: 100,
                            y: 100,
                            width: 800,
                            height: 600,
                          })
                        }
                        className="p-1 text-gray-600 hover:text-gray-800 transition-colors"
                        title="Update Bounds"
                      >
                        <ArrowsPointingOutIcon className="h-4 w-4" />
                      </button>
                      <button
                        onClick={() => handleHideWidget(widget.widget_id)}
                        className="p-1 text-gray-600 hover:text-gray-800 transition-colors"
                        title={widget.is_open ? "Hide Widget" : "Show Widget"}
                      >
                        {widget.is_open ? (
                          <EyeIcon className="h-4 w-4" />
                        ) : (
                          <EyeSlashIcon className="h-4 w-4" />
                        )}
                      </button>
                      <button
                        onClick={() => handleDeleteWidget(widget.widget_id)}
                        className="p-1 text-red-600 hover:text-red-800 transition-colors"
                        title="Delete Widget"
                      >
                        <TrashIcon className="h-4 w-4" />
                      </button>
                    </div>
                  </div>
                </div>

                {/* Widget Details */}
                {expandedWidgets.has(widget.widget_id) && (
                  <div className="border-t border-gray-200">
                    <div className="p-3 bg-gray-50">
                      <div className="grid grid-cols-2 gap-2 text-xs">
                        <div>
                          <p className="text-gray-500">Bounds</p>
                          {editingBounds?.widgetId === widget.widget_id ? (
                            <div className="space-y-2 mt-1">
                              <div className="grid grid-cols-2 gap-2">
                                <div>
                                  <label className="block text-xs text-gray-500">
                                    X
                                  </label>
                                  <input
                                    type="number"
                                    className="w-full border border-gray-300 rounded p-1 text-xs"
                                    value={editingBounds.bounds.x}
                                    onChange={(e) =>
                                      handleBoundsChange(
                                        "x",
                                        Number(e.target.value)
                                      )
                                    }
                                  />
                                </div>
                                <div>
                                  <label className="block text-xs text-gray-500">
                                    Y
                                  </label>
                                  <input
                                    type="number"
                                    className="w-full border border-gray-300 rounded p-1 text-xs"
                                    value={editingBounds.bounds.y}
                                    onChange={(e) =>
                                      handleBoundsChange(
                                        "y",
                                        Number(e.target.value)
                                      )
                                    }
                                  />
                                </div>
                              </div>
                              <div className="grid grid-cols-2 gap-2">
                                <div>
                                  <label className="block text-xs text-gray-500">
                                    Width
                                  </label>
                                  <input
                                    type="number"
                                    className="w-full border border-gray-300 rounded p-1 text-xs"
                                    value={editingBounds.bounds.width}
                                    onChange={(e) =>
                                      handleBoundsChange(
                                        "width",
                                        Number(e.target.value)
                                      )
                                    }
                                  />
                                </div>
                                <div>
                                  <label className="block text-xs text-gray-500">
                                    Height
                                  </label>
                                  <input
                                    type="number"
                                    className="w-full border border-gray-300 rounded p-1 text-xs"
                                    value={editingBounds.bounds.height}
                                    onChange={(e) =>
                                      handleBoundsChange(
                                        "height",
                                        Number(e.target.value)
                                      )
                                    }
                                  />
                                </div>
                              </div>
                              <div className="flex gap-2 mt-2">
                                <button
                                  onClick={handleSaveBounds}
                                  className="px-2 py-1 text-xs bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                                >
                                  Save
                                </button>
                                <button
                                  onClick={() => setEditingBounds(null)}
                                  className="px-2 py-1 text-xs text-gray-600 hover:text-gray-800 transition-colors"
                                >
                                  Cancel
                                </button>
                              </div>
                            </div>
                          ) : (
                            <div className="flex items-center gap-2">
                              <p className="text-gray-900">
                                {widget.bounds ? (
                                  <>
                                    {widget.bounds.x}, {widget.bounds.y} -{" "}
                                    {widget.bounds.width}×{widget.bounds.height}
                                  </>
                                ) : (
                                  "Not set"
                                )}
                              </p>
                              <button
                                onClick={() =>
                                  setEditingBounds({
                                    widgetId: widget.widget_id,
                                    bounds: widget.bounds || {
                                      x: 0,
                                      y: 0,
                                      width: 800,
                                      height: 600,
                                    },
                                  })
                                }
                                className="p-1 text-gray-400 hover:text-gray-600 transition-colors"
                                title="Edit Bounds"
                              >
                                <ArrowsPointingOutIcon className="h-3 w-3" />
                              </button>
                            </div>
                          )}
                        </div>
                        <div>
                          <p className="text-gray-500">Level</p>
                          <p className="text-gray-900">{widget.level}</p>
                        </div>
                        <div>
                          <p className="text-gray-500">Type</p>
                          <p className="text-gray-900">
                            {widget.widget_type.type}
                          </p>
                        </div>
                        <div>
                          <p className="text-gray-500">Settings</p>
                          <p className="text-gray-900">
                            {widget.transparent ? "Transparent" : "Opaque"}
                            {widget.decorations
                              ? ", Decorated"
                              : ", Undecorated"}
                          </p>
                        </div>
                      </div>
                    </div>
                  </div>
                )}

                {/* Modifiers Section */}
                {expandedWidgets.has(widget.widget_id) && (
                  <div className="border-t border-gray-200">
                    <div className="p-3 bg-gray-50">
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="text-xs font-medium text-gray-700">
                          Modifiers
                        </h4>
                        <button
                          onClick={() => handleAddModifier(widget.widget_id)}
                          className="flex items-center gap-1 text-xs text-blue-600 hover:text-blue-800 transition-colors"
                        >
                          <PlusIcon className="h-3 w-3" />
                          <span>Add Modifier</span>
                        </button>
                      </div>

                      {widgetModifiers[widget.widget_id]?.length > 0 ? (
                        <div className="space-y-2">
                          {widgetModifiers[widget.widget_id].map((modifier) => (
                            <div
                              key={modifier.id}
                              className="bg-white p-2 rounded border border-gray-200 flex items-center justify-between group hover:border-gray-300 transition-colors"
                            >
                              <div className="flex items-center gap-2">
                                {modifier.modifier_type.type === "scrape" ? (
                                  <div className="p-1.5 bg-blue-50 rounded">
                                    <CodeBracketIcon className="h-4 w-4 text-blue-600" />
                                  </div>
                                ) : (
                                  <div className="p-1.5 bg-purple-50 rounded">
                                    <ClockIcon className="h-4 w-4 text-purple-600" />
                                  </div>
                                )}
                                <div>
                                  <div className="text-xs font-medium text-gray-900">
                                    {modifier.modifier_type.type === "scrape"
                                      ? "Scrape Element"
                                      : "Auto Refresh"}
                                  </div>
                                  {modifier.modifier_type.type === "scrape" && (
                                    <div className="text-xs text-gray-500">
                                      {modifier.modifier_type.content.selector}
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
                                className="p-1 text-gray-400 hover:text-red-600 transition-colors opacity-0 group-hover:opacity-100"
                                title="Delete Modifier"
                              >
                                <TrashIcon className="h-3 w-3" />
                              </button>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="text-center py-4 bg-white rounded border border-gray-200">
                          <div className="w-8 h-8 mx-auto mb-2 rounded-full bg-blue-50 flex items-center justify-center">
                            <PlusIcon className="h-4 w-4 text-blue-600" />
                          </div>
                          <p className="text-xs text-gray-500 mb-1">
                            No modifiers added yet
                          </p>
                          <button
                            onClick={() => handleAddModifier(widget.widget_id)}
                            className="text-xs text-blue-600 hover:text-blue-800 transition-colors"
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
            <div className="text-center py-8 bg-white rounded-lg border border-gray-200">
              <div className="w-12 h-12 mx-auto mb-2 rounded-full bg-blue-50 flex items-center justify-center">
                <PlusIcon className="h-6 w-6 text-blue-600" />
              </div>
              <p className="text-sm text-gray-900 mb-1">No widgets found</p>
              <p className="text-xs text-gray-500">
                Create a new widget to get started
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Modifier Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-25 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg w-[28rem] max-w-[90vw] shadow-xl">
            <h3 className="text-lg font-semibold mb-4">Add Modifier</h3>

            {/* Modifier Type Selection */}
            <div className="mb-4">
              <label className="block text-xs font-medium mb-1 text-gray-700">
                Modifier Type
              </label>
              <select
                className="w-full border border-gray-300 rounded p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
                <label className="block text-xs font-medium mb-1 text-gray-700">
                  CSS Selector
                </label>
                <input
                  type="text"
                  className="w-full border border-gray-300 rounded p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
              <div className="mb-4">
                <label className="block text-xs font-medium mb-1 text-gray-700">
                  Refresh Interval (seconds)
                </label>
                <input
                  type="number"
                  className="w-full border border-gray-300 rounded p-2 text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
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
            <div className="flex justify-end space-x-2">
              <button
                onClick={() => setIsModalOpen(false)}
                className="px-3 py-1.5 text-sm text-gray-600 hover:text-gray-800 transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={handleSaveModifier}
                className="px-3 py-1.5 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
                disabled={!modifierType}
              >
                Save
              </button>
            </div>
          </div>
        </div>
      )}

      {toast && (
        <Toast
          message={toast.message}
          type={toast.type}
          onClose={() => setToast(null)}
        />
      )}
    </div>
  );
}
