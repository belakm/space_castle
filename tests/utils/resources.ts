// Assets to be created and funded to the Liquidity Pool
// (Name, decimals, quantity, URI)

export interface MarketResourceMetadata {
  name: string
  symbol: string
  decimals: number
  quantity: number
  mintKey: string
}

export const MARKET_RESOURCES: Array<MarketResourceMetadata> = [
  {
    name: 'Intergalactic Tender',
    symbol: 'IGT',
    decimals: 9,
    quantity: 10000000,
    mintKey: 'igt',
  },
  {
    name: 'Metals',
    symbol: 'rMET',
    decimals: 2,
    quantity: 10000,
    mintKey: 'metal',
  },
  {
    name: 'Metals',
    symbol: 'rCHEM',
    decimals: 6,
    quantity: 10000,
    mintKey: 'chemical',
  },
]
