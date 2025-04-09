import { useEffect, useState, useCallback } from "react";
import { AppSettings, IpcEvent } from "./types";
import AutoForm from "./AutoForm";

// make a default settings object
const defaultSettings: AppSettings = {
  show_tray_icon: true,
};
export default function SettingsWidget() {
  const [appSettings, setAppSettings] = useState<AppSettings>(defaultSettings);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isDirty, setIsDirty] = useState(false);

  useEffect(() => {
    setIsLoading(true);
    setError(null);
    setAppSettings(defaultSettings);
    setIsLoading(false);
    setIsDirty(false);
  }, []);

  const handleSettingsChange = useCallback(
    (updatedData: Partial<AppSettings>) => {
      setAppSettings((prevSettings) => ({
        ...prevSettings,
        ...updatedData,
      }));
      setIsDirty(true);
    },
    []
  );

  const handleSave = async () => {
    setIsLoading(true);
    setError(null);
    console.log("Saving settings:", appSettings);
    try {
      const ipc_event: IpcEvent = {
        type: "SaveSettings",
        content: appSettings,
      };
      window.ipc.postMessage(JSON.stringify(ipc_event));
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "An unknown error occurred"
      );
    }
    setIsLoading(false);
    setIsDirty(false);
  };

  return (
    <div className="p-4 max-w-lg mx-auto">
      <h2 className="text-xl font-bold mb-4">Application Settings</h2>
      {isLoading && <p>Loading settings...</p>}
      {error && <p className="text-red-500 mb-3">Error: {error}</p>}
      {!isLoading && !error && (
        <>
          <AutoForm data={appSettings} onChange={handleSettingsChange} />
          <div className="mt-4 flex justify-end">
            <button
              onClick={handleSave}
              disabled={isLoading || !isDirty}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
            >
              {isLoading ? "Saving..." : "Save Settings"}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
