/**
 *
 * Returns the real quantity of a `quantity` parameter by
 * increasing the number using the mint's decimal places
 *
 * @param quantity The provided quantity argument
 * @param decimals The decimals of the associated mint
 * @returns The real quantity of a `quantity` parameter
 */
export function toBigIntQuantity(quantity: number, decimals: number): bigint {
  return BigInt(quantity) * BigInt(10) ** BigInt(decimals)
}

/**
 *
 * Returns the nominal quantity of a `quantity` parameter by
 * decreasing the number using the mint's decimal places
 *
 * @param quantity The real quantity of a `quantity` parameter
 * @param decimals The decimals of the associated mint
 * @returns The nominal quantity of a `quantity` parameter
 */
export function fromBigIntQuantity(quantity: bigint, decimals: number): string {
  return (Number(quantity) / 10 ** decimals).toFixed(6)
}
