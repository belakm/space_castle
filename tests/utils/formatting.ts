export function number_to_bytes_buffer(input: number) {
  return new Uint8Array(new Uint16Array([input]).buffer)
}
