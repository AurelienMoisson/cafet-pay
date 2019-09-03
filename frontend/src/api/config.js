export default config() {
  const ret = {}
  switch (process.env.NODE_ENV) {
    case "development":
      ret.cardReaderUrl = "localhost:3002/"
      ret.accountsApiUrl = "?"
    default:
      ret.cardReaderUrl = "localhost:3002/"
      ret.accountsApiUrl = "?"
  }
  return ret
}
