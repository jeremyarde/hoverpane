import { useEffect, useState } from "react";
import { ScrapedData } from "./types";

export default function DataWidget() {
  const [values, setValues] = useState<ScrapedData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filterText, setFilterText] = useState("");

  const fetchValues = async () => {
    try {
      const response = await fetch(`http://localhost:3111/values`);
      if (!response.ok) {
        throw new Error("Failed to fetch values");
      }
      const data = await response.json();
      setValues(data);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch values");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchValues();
    const interval = setInterval(fetchValues, 5000);
    return () => clearInterval(interval);
  }, []);

  const filteredValues = values.filter((value) => {
    const searchText = filterText.toLowerCase();
    return (
      value.widget_id.toLowerCase().includes(searchText) ||
      value.value.toLowerCase().includes(searchText) ||
      new Date(parseInt(value.timestamp))
        .toLocaleString()
        .toLowerCase()
        .includes(searchText) ||
      (value.error?.toLowerCase() || "success").includes(searchText)
    );
  });

  if (loading) {
    return <div className="p-4">Loading...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">Error: {error}</div>;
  }

  return (
    <div className="p-4 h-full flex flex-col">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-bold">Scraped Data</h2>
        <div className="relative">
          <input
            type="text"
            placeholder="Filter data..."
            value={filterText}
            onChange={(e) => setFilterText(e.target.value)}
            className="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 w-64"
          />
          {filterText && (
            <button
              onClick={() => setFilterText("")}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
            >
              ×
            </button>
          )}
        </div>
      </div>
      <div className="flex-1 overflow-auto rounded-lg border border-gray-200">
        <table className="min-w-full divide-y divide-gray-200 bg-white border-separate border-spacing-x-4">
          <thead className="bg-gray-50 sticky top-0">
            <tr>
              <th
                scope="col"
                className="px-8 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider whitespace-nowrap bg-gray-50"
              >
                Widget ID
              </th>
              <th
                scope="col"
                className="px-8 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider whitespace-nowrap bg-gray-50"
              >
                Value
              </th>
              <th
                scope="col"
                className="px-8 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider whitespace-nowrap bg-gray-50"
              >
                Timestamp
              </th>
              <th
                scope="col"
                className="px-8 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider whitespace-nowrap bg-gray-50"
              >
                Status
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white">
            {filteredValues.map((value) => (
              <tr
                key={`${value.widget_id}-${value.timestamp}`}
                className="hover:bg-gray-50"
              >
                <td className="px-8 py-4 text-sm text-gray-900 whitespace-nowrap">
                  {value.widget_id}
                </td>
                <td className="px-8 py-4 text-sm text-gray-900 whitespace-nowrap">
                  {value.value}
                </td>
                <td className="px-8 py-4 text-sm text-gray-500 whitespace-nowrap">
                  {new Date(parseInt(value.timestamp)).toLocaleString()}
                </td>
                <td className="px-8 py-4 text-sm whitespace-nowrap">
                  {value.error ? (
                    <span className="text-red-500">Error: {value.error}</span>
                  ) : (
                    <span className="text-green-500">Success</span>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {filteredValues.length === 0 && (
          <div className="text-center text-gray-500 p-4">
            {values.length === 0 ? "No data available" : "No matching results"}
          </div>
        )}
      </div>
    </div>
  );
}
