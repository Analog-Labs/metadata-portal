import analogSrc from "./assets/icons/analog.svg";

const ICONS = {
  analog: analogSrc,
  "analog-testnet": analogSrc,
  "analog-timechain": analogSrc,
};

export function icon(network: string) {
  return ICONS[network as "analog"];
}
