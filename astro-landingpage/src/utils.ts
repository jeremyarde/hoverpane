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

    const link = document.createElement("a");
    link.href = data.download_url;
    const filename = data.download_url.split("/").pop() || "hoverpane-download";
    link.setAttribute("download", filename);
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  } catch (error) {
    console.error("Download failed:", error);
  }
};

export { handleDownload };
