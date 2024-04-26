export const dateFormat = {
  month: "2-digit",
  day: "numeric",
  year: "2-digit",
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
  hour12: false,
};

export const dateFormatNoSecs = {
  month: "2-digit",
  day: "numeric",
  year: "2-digit",
  hour: "2-digit",
  minute: "2-digit",
  hour12: false,
};

export const formatEpoch = (epochTime: string, includeSecs: boolean = false) => {
  if (epochTime != null) {
    try {
      return new Date(parseInt(epochTime)).toLocaleString(undefined, includeSecs ? dateFormat : dateFormatNoSecs);
    } catch (error) {
      return epochTime;
    }
  }
  return epochTime;
};
