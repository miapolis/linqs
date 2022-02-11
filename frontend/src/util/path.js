export const main = () => {
  return window.location.origin;
};

export const apiBase = () => {
  if (import.meta.env.PUBLIC_API_URL) {
    return import.meta.env.PUBLIC_API_URL;
  }
  return main();
};

export const api = () => {
  return `${apiBase()}/api`;
};
