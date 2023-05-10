export const dateFormat = {
  month: "numeric",
  day: "numeric",
  hour: "numeric",
  minute: "numeric",
  second: "numeric",
  hour12: false,
};

export const formatEpoch = (epochTime: string) => {
  if (epochTime != null) {
    try {
      return new Date(parseInt(epochTime) / 1000).toLocaleString(undefined, dateFormat);
    } catch (error) {
      return epochTime;
    }
  }
  return epochTime;
};
