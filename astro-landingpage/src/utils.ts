import { HOVERPANE_DOWNLOAD_URL } from "./landingpage/constants";

type DownloadUrlResponse = {
  download_url: string;
};

const getBaseUrl = () => {
  if (import.meta.env.PROD) {
    return "https://hoverpane.com";
  }
  return "http://localhost:3001";
};

const getDownloadUrl = async (url: string): Promise<DownloadUrlResponse> => {
  const baseUrl = getBaseUrl();
  const fullUrl = `${baseUrl}${url}`;
  const res = await fetch(fullUrl);
  const data = await res.json();
  return data;
};

const handleDownload = async () => {
  try {
    const data = await getDownloadUrl(HOVERPANE_DOWNLOAD_URL);
    const newWindow = window.open(data.download_url, "_blank");
    if (newWindow) {
      newWindow.opener = null;
    }
  } catch (error) {
    console.error("Download failed:", error);
  }
};

export { getDownloadUrl, handleDownload };
