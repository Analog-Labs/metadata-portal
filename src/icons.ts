import analogDevelopSrc from "./assets/icons/analog-develop.svg";
import analogTestnetSrc from "./assets/icons/analog-testnet.svg";
import analogTimechainSrc from "./assets/icons/analog-timechain.svg";

const ICONS = {
  analog: analogDevelopSrc,
  "analog-testnet": analogTestnetSrc,
  "analog-timechain": analogTimechainSrc,
};

export function icon(network: string) {
  return ICONS[network as "analog"];
}
