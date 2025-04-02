import { useEffect, useState } from "react";

interface ScrapedValue {
  id: number;
  widget_id: {
    0: string; // NanoId wrapped value
  };
  value: string;
  error: string | null;
  timestamp: string;
}

export default function DataWidget() {
  const [values, setValues] = useState<ScrapedValue[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchValues = async () => {
    try {
      const response = await fetch("http://localhost:3000/values");
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
    // Refresh data every 5 seconds
    const interval = setInterval(fetchValues, 5000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return <div className="p-4">Loading...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">Error: {error}</div>;
  }

  return (
    <div className="p-4">
      <h2 className="text-xl font-bold mb-4">Scraped Data</h2>
      <div className="relative max-h-[600px] overflow-auto border border-gray-200 rounded-lg">
        <table className="w-full bg-white">
          <thead className="sticky top-0 z-10">
            <tr className="bg-gray-50 border-b border-gray-200">
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Widget ID
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Value
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Timestamp
              </th>
              <th
                scope="col"
                className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Status
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {values.map((value) => (
              <tr key={`${value.id}-${value.timestamp}`}>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {value.widget_id[0]}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {value.value}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {new Date(value.timestamp).toLocaleString()}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm">
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
        {values.length === 0 && (
          <div className="text-center text-gray-500 p-4">No data available</div>
        )}
      </div>
    </div>
  );
}
