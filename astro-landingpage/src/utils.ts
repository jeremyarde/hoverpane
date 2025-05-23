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

export { getDownloadUrl };
