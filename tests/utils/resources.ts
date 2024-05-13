// Assets to be created and funded to the Liquidity Pool
// (Name, decimals, quantity, URI)

export interface MarketResourceMetadata {
  name: string
  symbol: string
  decimals: number
  quantity: number
  mintKey: ResourceKey
}

export const MARKET_RESOURCES: Array<MarketResourceMetadata> = [
  {
    name: 'Intergalactic Tender',
    symbol: 'iGT',
    decimals: 8,
    quantity: 10000000,
    mintKey: 'igt',
  },
  {
    name: 'Metals',
    symbol: 'rMETL',
    decimals: 8,
    quantity: 10000,
    mintKey: 'metal',
  },
  {
    name: 'Chemicals',
    symbol: 'rCHEM',
    decimals: 8,
    quantity: 10000,
    mintKey: 'chemical',
  },
  {
    name: 'Fuel',
    symbol: 'rFUEL',
    decimals: 8,
    quantity: 10000,
    mintKey: 'fuel',
  },
  {
    name: 'Crystals',
    symbol: 'rCRYS',
    decimals: 8,
    quantity: 10000,
    mintKey: 'crystal',
  },
]

export type ResourceKey = 'igt' | 'metal' | 'chemical' | 'crystal' | 'fuel'
