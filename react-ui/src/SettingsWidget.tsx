import { useEffect, useState, useCallback } from "react";
import { AppSettings, AppUiState, LicenceTier } from "./types";
import { getAppUiState, getSettings, setSettings } from "./clientInterface";
// import { API_URL, CREATE_PURCHASE_PATH } from "./constants";

// make a default settings object
const defaultSettings: AppSettings = {
  show_tray_icon: true,
  email: "",
  licence_key: "",
  machine_id: "",
  licence_tier: LicenceTier.Free,
};

const defaultAppUiState: AppUiState = {
  app_settings: defaultSettings,
  messages: [],
};

export default function SettingsWidget() {
  const [appSettings, setAppSettings] = useState<AppSettings>(defaultSettings);
  const [appUiState, setAppUiState] = useState<AppUiState>(defaultAppUiState);
  const [isLoading, setIsLoading] = useState(false);
  const [emailTouched, setEmailTouched] = useState(false);
  const [licenceTouched, setLicenceTouched] = useState(false);

  const isEmailValid = appSettings.email.trim().length > 0;
  const isLicenceValid = appSettings.licence_key.trim().length > 0;
  const canSave = isEmailValid && isLicenceValid;

  // Fetch settings from the app on mount
  useEffect(() => {
    setIsLoading(true);
    Promise.all([getSettings(), getAppUiState()])
      .then(([settings, appUiState]) => {
        setAppSettings(settings);
        setAppUiState(appUiState);
        setIsLoading(false);
      })
      .catch(() => {
        setIsLoading(false);
      });
  }, []);

  const handleSettingsChange = useCallback(
    (updatedData: Partial<AppSettings>) => {
      setAppSettings((prevSettings) => ({
        ...prevSettings,
        ...updatedData,
      }));
    },
    []
  );

  // Handler for licence_key input
  const handleLicenceChange = (value: string) => {
    setAppSettings((prev) => ({ ...prev, licence_key: value }));
  };
  const handleLicenceBlur = () => {
    setLicenceTouched(true);
  };

  return (
    <div className="p-1 mx-auto max-w-md text-sm">
      {/* <h2 className="mb-2 text-lg font-bold">Settings</h2> */}
      {/* {isLoading && <p>Saving…</p>} */}
      {/* {error && <p className="mb-2 text-red-500">Error: {error}</p>} */}
      {appUiState.messages?.map((message) => (
        <p className="mb-2 text-green-500" key={message}>
          {message}
        </p>
      ))}
      {!isLoading && (
        <>
          <div className="flex items-center mb-2">
            <input
              type="checkbox"
              id="show_tray_icon"
              className="mr-1 w-4 h-4"
              checked={appSettings.show_tray_icon}
              onChange={(e) =>
                handleSettingsChange({ show_tray_icon: e.target.checked })
              }
            />
            <label htmlFor="show_tray_icon" className="cursor-pointer">
              Show Tray Icon
            </label>
          </div>
          <div className="grid grid-cols-1 gap-2 pt-2 border-t">
            <div className="flex gap-2 items-center">
              <label htmlFor="user_email" className="w-20">
                Email
              </label>
              <input
                type="email"
                id="user_email"
                className="flex-1 px-1 py-0.5 rounded border border-gray-300 text-xs"
                value={appSettings.email}
                onChange={(e) =>
                  handleSettingsChange({ email: e.target.value })
                }
                onBlur={() => setEmailTouched(true)}
                required
              />
            </div>
            {!isEmailValid && emailTouched && (
              <div className="ml-20 text-xs text-red-500">
                Email is required.
              </div>
            )}
            <div className="flex gap-2 items-center">
              <label htmlFor="licence_key" className="w-20">
                Licence
              </label>
              <input
                type="text"
                id="licence_key"
                className="flex-1 px-1 py-0.5 rounded border border-gray-300 text-xs"
                value={appSettings.licence_key}
                onChange={(e) => handleLicenceChange(e.target.value)}
                onBlur={handleLicenceBlur}
              />
            </div>
            {!isLicenceValid && licenceTouched && (
              <div className="ml-20 text-xs text-red-500">
                Licence key is required.
              </div>
            )}
            <div className="flex gap-2 items-center">
              <span className="text-gray-600">Tier:</span>
              <span className="px-1 py-0.5 text-xs font-medium bg-gray-100 rounded">
                {appSettings.licence_tier}
              </span>
              <button
                className="px-2 py-0.5 text-xs text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50"
                onClick={async () => {
                  window.ipc.postMessage(
                    JSON.stringify({
                      type: "checklicence",
                      content: {
                        email: appSettings.email,
                        licence_key: appSettings.licence_key,
                      },
                    })
                  );
                  const appUiState = await getAppUiState();
                  setAppUiState(appUiState);
                }}
              >
                Verify
              </button>
              {appSettings.licence_tier === LicenceTier.Free && (
                <button
                  className="px-2 py-0.5 text-xs text-white bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50"
                  onClick={async () => {
                    window.ipc.postMessage(
                      JSON.stringify({
                        type: "buylicence",
                        content: {
                          email: appSettings.email,
                        },
                      })
                    );
                    const appUiState = await getAppUiState();
                    setAppUiState(appUiState);
                  }}
                >
                  Buy Pro
                </button>
              )}
            </div>
          </div>
          <div className="flex justify-end mt-4">
            <button
              className="px-3 py-1 text-xs text-white bg-green-600 rounded hover:bg-green-700 disabled:opacity-50"
              disabled={!canSave || isLoading}
              onClick={async () => {
                setIsLoading(true);
                try {
                  await setSettings(appSettings);
                } catch {
                  console.log("Error saving settings");
                }
                setIsLoading(false);
              }}
            >
              {isLoading ? "Saving…" : "Save"}
            </button>
          </div>
        </>
      )}
    </div>
  );
}
