import React, { useState } from "react";
import "./styles.css";

interface CheckoutResponse {
  checkoutUrl: string;
}

const Purchase: React.FC = () => {
  const [email, setEmail] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [checkoutUrl, setCheckoutUrl] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch("/api/create-checkout", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
      });

      if (!response.ok) {
        throw new Error("Failed to create checkout session");
      }

      const data: CheckoutResponse = await response.json();
      setCheckoutUrl(data.checkoutUrl);
    } catch (err) {
      setError(err instanceof Error ? err.message : "An error occurred");
    } finally {
      setIsLoading(false);
    }
  };

  if (checkoutUrl) {
    window.location.href = checkoutUrl;
    return null;
  }

  return (
    <div className="flex justify-center items-center px-4 min-h-screen bg-slate-50">
      <div className="p-8 w-full max-w-md bg-white rounded-xl shadow-lg">
        <h1 className="mb-6 text-3xl font-bold text-center text-slate-800">
          Get Started with HoverPane
        </h1>
        <p className="mb-8 text-center text-slate-600">
          Enter your email to continue to checkout and start using HoverPane
          today.
        </p>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label
              htmlFor="email"
              className="block mb-2 text-sm font-medium text-slate-700"
            >
              Email Address
            </label>
            <input
              type="email"
              id="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="px-4 py-2 w-full rounded-lg border focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 border-slate-300"
              placeholder="you@example.com"
              required
            />
          </div>

          {error && (
            <div className="p-3 text-sm text-red-600 bg-red-50 rounded-lg">
              {error}
            </div>
          )}

          <button
            type="submit"
            disabled={isLoading}
            className="px-6 py-3 w-full font-semibold text-white bg-indigo-600 rounded-lg transition-all hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? "Processing..." : "Continue to Checkout"}
          </button>
        </form>

        <p className="mt-6 text-sm text-center text-slate-500">
          By continuing, you agree to our Terms of Service and Privacy Policy.
        </p>
      </div>
    </div>
  );
};

export default Purchase;
