export const main = () => {
  return window.location.origin;
};

export const api = () => {
  if (import.meta.env.PUBLIC_API_URL) {
    return import.meta.env.PUBLIC_API_URL;
  }
  return `${main()}/api`;
};
