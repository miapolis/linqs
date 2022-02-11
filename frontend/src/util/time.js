export const utcTimestamp = () => {
  const d = new Date();
  return d.getTime() + d.getTimezoneOffset() * 60 * 1000;
};

export const formatDifference = (one, two) => {
  let diff = one.getTime() - two.getTime();

  let months;
  months = one.getFullYear() - two.getFullYear();
  months -= one.getMonth();
  months += two.getMonth();

  months = months <= 0 ? 0 : months;

  const days = Math.floor(diff / 1000 / 60 / 60 / 24);
  diff -= days * 1000 * 60 * 60 * 24;

  const hours = Math.floor(diff / 1000 / 60 / 60);
  diff -= hours * 1000 * 60 * 60;

  const minutes = Math.floor(diff / 1000 / 60);
  diff -= minutes * 1000 * 60;

  const seconds = Math.floor(diff / 1000);

  return `${months > 0 ? `${months} months, ` : ""}${
    days > 0 ? `${days} day${days > 1 ? "s" : ""}, ` : ""
  }${hours > 0 ? `${hours} hr, ` : ""}${minutes > 0 ? `${minutes} min, ` : ""}${
    seconds > 0 ? `${seconds} sec` : ""
  }`;
};
