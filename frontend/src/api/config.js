export default config() {
  const ret = {}
  switch (process.env.NODE_ENV) {
    case "development":
      ret.cardReaderUrl = "localhost:3002/"
    default:
      ret.cardReaderUrl = "localhost:3002/"
  }
  return ret
}
