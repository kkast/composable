export type AvailableEndpoints = "picasso" | "kusama" | "karura" | "subsquid";

function cacheAndFetch(target: "picasso" | "kusama" | "karura" | "subsquid") {
  let cache = "";
  switch (target) {
    case "kusama":
      cache = process.env.SUBSTRATE_PROVIDER_URL_KUSAMA || "";
      break;
    case "picasso":
      cache = process.env.SUBSTRATE_PROVIDER_URL_KUSAMA_2019 || "";
      break;
    case "karura":
      cache = process.env.SUBSTRATE_PROVIDER_URL_KARURA || "";
      break;
    case "subsquid":
      cache = process.env.SUBSQUID_URL || "";
      break;
  }

  if (typeof localStorage !== "undefined") {
    localStorage.setItem(getLocalStorageCacheKey(target), cache);
  }

  return cache;
}

export function getLocalStorageCacheKey(target: AvailableEndpoints) {
  return `${target}-endpoint`;
}

export function getEnvironment(target: AvailableEndpoints) {
  if (typeof localStorage !== "undefined") {
    const cachedValue = localStorage.getItem(getLocalStorageCacheKey(target));
    if (cachedValue) return cachedValue;
  }

  return cacheAndFetch(target);
}

export type EndpointPreset = "local" | "daliStage" | "picassoStage";
export type EndpointPresets = {
  [key in EndpointPreset]: {
    [key in AvailableEndpoints]: string;
  };
};
export const endpointPresets: EndpointPresets = {
  local: {
    picasso: "ws://127.0.0.1:9988",
    karura: "ws://127.0.0.1:9999",
    kusama: "ws://127.0.0.1:9944",
    subsquid: "http://localhost:4350/graphql",
  },
  daliStage: {
    picasso: "wss://persistent.devnets.composablefinance.ninja/chain/dali",
    karura: "wss://persistent.devnets.composablefinance.ninja/chain/karura",
    kusama: "wss://persistent.devnets.composablefinance.ninja/chain/rococo",
    subsquid:
      "https://persistent.devnets.composablefinance.ninja/subsquid/graphql",
  },
  picassoStage: {
    picasso:
      "wss://persistent.picasso.devnets.composablefinance.ninja/chain/picasso",
    karura:
      "wss://persistent.picasso.devnets.composablefinance.ninja/chain/karura",
    kusama:
      "wss://persistent.picasso.devnets.composablefinance.ninja/chain/rococo",
    subsquid:
      "https://persistent.picasso.devnets.composablefinance.ninja/subsquid/graphql",
  },
};

export function setEndpointPreset(endpointPreset: EndpointPreset) {
  Object.entries(endpointPresets[endpointPreset]).forEach(([target, value]) => {
    if (typeof localStorage !== "undefined") {
      localStorage.setItem(
        getLocalStorageCacheKey(target as AvailableEndpoints),
        value
      );
    }
  });
}