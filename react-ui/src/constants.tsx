// export const API_URL = "http://127.0.0.1:3111";
// export const LICENCE_CHECK_URL = "http://127.0.0.1:3001/licence/check";
// export const CREATE_PURCHASE_URL =
//   "http://127.0.0.1:3001/stripe/generate-stripe-checkout";

// export const API_URL = getBaseUrl();
export const CREATE_PURCHASE_PATH = "/stripe/generate-stripe-checkout";
export const LICENCE_CHECK_PATH = "/licence/check";

const getBaseUrl = () => {
  if (import.meta.env.PROD) {
    return "https://api.hoverpane.com";
  }
  return "http://localhost:3000";
};

export const API_URL = getBaseUrl();
