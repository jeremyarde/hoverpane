import { useState } from "react";
import { ArrowRightIcon } from "@heroicons/react/24/outline";

interface CheckoutProps {
  onSuccess?: () => void;
  onError?: (error: string) => void;
}

export function Checkout({ onSuccess, onError }: CheckoutProps) {
  const [isLoading, setIsLoading] = useState(false);

  const handleCheckout = async () => {
    try {
      setIsLoading(true);
      // Replace with your actual backend endpoint
      const response = await fetch(
        "http://localhost:3000/api/create-checkout",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
        }
      );

      if (!response.ok) {
        throw new Error("Failed to create checkout session");
      }

      const { url } = await response.json();

      // Redirect to the checkout URL
      window.location.href = url;

      onSuccess?.();
    } catch (error) {
      console.error("Checkout error:", error);
      onError?.(
        error instanceof Error
          ? error.message
          : "An error occurred during checkout"
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center p-6 bg-white rounded-lg shadow-lg">
      <h2 className="text-2xl font-bold mb-4">Upgrade to Pro</h2>
      <p className="text-gray-600 mb-6 text-center">
        Get access to all premium features and unlimited widgets
      </p>
      <button
        onClick={handleCheckout}
        disabled={isLoading}
        className="flex items-center justify-center px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isLoading ? (
          "Processing..."
        ) : (
          <>
            Continue to Checkout
            <ArrowRightIcon className="w-5 h-5 ml-2" />
          </>
        )}
      </button>
    </div>
  );
}
