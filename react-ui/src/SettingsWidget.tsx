import { useEffect, useState, useCallback } from "react";
import { AppSettings, IpcEvent, LicenceTier } from "./types";
import { checkLicence } from "./clientInterface";
import { CREATE_PURCHASE_URL } from "./constants";

// make a default settings object
const defaultSettings: AppSettings = {
  show_tray_icon: true,
  user_email: "",
  licence_key: "",
  machine_id: "",
  licence_tier: LicenceTier.Free,
};

export default function SettingsWidget() {
  const [appSettings, setAppSettings] = useState<AppSettings>(defaultSettings);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isDirty, setIsDirty] = useState(false);
  const [emailTouched, setEmailTouched] = useState(false);

  const isEmailValid = appSettings.user_email.trim().length > 0;

  // Fetch settings from the app on mount
  useEffect(() => {
    setIsLoading(true);
    setError(null);
    // Handler for receiving settings from Rust backend
    const handleRustMessage = (element: string) => {
      try {
        const msg = JSON.parse(element);
        // Heuristic: look for settings in message
        if (msg.data_key === "settings" && msg.message) {
          const settings = JSON.parse(msg.message) as AppSettings;
          setAppSettings(settings);
          setIsLoading(false);
          setIsDirty(false);
        }
      } catch {
        // Ignore unrelated messages
      }
    };
    // Attach handler
    window.onRustMessage = handleRustMessage;
    // Request settings from backend
    window.ipc.postMessage(JSON.stringify({ type: "getsettings" }));
    // Cleanup
    return () => {
      window.onRustMessage = () => {};
    };
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
    setEmailTouched(true);
    if (!isEmailValid) {
      setError("Email is required.");
      return;
    }
    setIsLoading(true);
    setError(null);
    console.log("Saving settings:", appSettings);
    try {
      const ipc_event: IpcEvent = {
        type: "savesettings",
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
    <div className="p-4 mx-auto max-w-lg">
      <h2 className="mb-4 text-xl font-bold">Application Settings</h2>
      {isLoading && <p>Loading settings...</p>}
      {error && <p className="mb-3 text-red-500">Error: {error}</p>}
      {!isLoading && !error && (
        <>
          {/* <AutoForm
            data={{
              show_tray_icon: appSettings.show_tray_icon,
              user_email: appSettings.user_email,
            }}
            onChange={handleSettingsChange}
          /> */}
          <input
            type="checkbox"
            className="mr-2"
            checked={appSettings.show_tray_icon}
            onChange={(e) =>
              handleSettingsChange({ show_tray_icon: e.target.checked })
            }
          />
          <label htmlFor="show_tray_icon">Show Tray Icon</label>
          <div>
            <hr className="my-4" />
            <h3 className="mb-2 text-lg font-bold">Login</h3>
            <div className="space-y-4">
              <div className="flex items-center">
                <label htmlFor="user_email" className="w-24">
                  Email
                </label>
                <input
                  type="email"
                  className="flex-1 px-2 py-1 rounded-md border-2 border-gray-300"
                  value={appSettings.user_email}
                  onChange={(e) =>
                    handleSettingsChange({ user_email: e.target.value })
                  }
                  onBlur={() => setEmailTouched(true)}
                  required
                />
              </div>
              {!isEmailValid && emailTouched && (
                <div className="mt-1 ml-24 text-sm text-red-500">
                  Email is required.
                </div>
              )}
              <div className="flex items-center">
                <label htmlFor="licence_key" className="w-24">
                  Licence Key
                </label>
                <input
                  type="text"
                  className="flex-1 px-2 py-1 rounded-md border-2 border-gray-300"
                  value={appSettings.licence_key}
                  onChange={(e) =>
                    handleSettingsChange({ licence_key: e.target.value })
                  }
                />
              </div>
              <div className="flex justify-between items-center">
                <div className="flex items-center space-x-2">
                  <span className="text-sm text-gray-600">Current Tier:</span>
                  <span className="px-2 py-1 text-sm font-medium bg-gray-100 rounded">
                    {appSettings.licence_tier}
                  </span>
                </div>
                <button
                  className="px-4 py-1 text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50"
                  onClick={() => {
                    console.log("Checking licence");
                    checkLicence(
                      window.WIDGET_ID,
                      appSettings.user_email,
                      appSettings.licence_key
                    );
                  }}
                >
                  Verify Licence
                </button>
                {appSettings.licence_tier === LicenceTier.Free && (
                  <button
                    className="px-4 py-1 text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50"
                    onClick={async () => {
                      const res = await fetch(CREATE_PURCHASE_URL, {
                        method: "POST",
                        body: JSON.stringify({
                          email: appSettings.user_email,
                        }),
                        headers: {
                          "Content-Type": "application/json",
                        },
                      });
                      if (res.ok) {
                        const data = await res.json();
                        console.log("Purchase successful", data);
                        if (data.checkout_session_url) {
                          window.open(data.checkout_session_url, "_blank");
                        }
                      } else {
                        const data = await res.json();
                        console.error("Failed to purchase licence", data);
                      }
                    }}
                  >
                    Buy Pro
                  </button>
                )}
              </div>
            </div>
          </div>
          <div className="flex justify-end mt-4">
            <button
              onClick={handleSave}
              disabled={isLoading || !isDirty || !isEmailValid}
              className="px-4 py-2 text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50"
            >
              {isLoading ? "Saving..." : "Save Settings"}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
