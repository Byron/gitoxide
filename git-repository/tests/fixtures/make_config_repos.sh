set -eu -o pipefail

git init http-config
(cd http-config
  git config http.extraHeader "ExtraHeader: value1"
  git config --add http.extraHeader ""
  git config --add http.extraHeader "ExtraHeader: value2"
  git config --add http.extraHeader "ExtraHeader: value3"
  git config http.followRedirects initial
  git config http.lowSpeedLimit 5k
  git config http.lowSpeedTime 10
  git config http.postBuffer 8k
  git config http.proxy http://localhost:9090
  git config http.proxyAuthMethod anyauth
  git config http.userAgent agentJustForHttp
)

git init http-proxy-empty
(cd http-proxy-empty
  git config http.proxy localhost:9090
  git config --add http.proxy "" # a value override disabling it later
)

git init http-proxy-auto-prefix
(cd http-proxy-auto-prefix
  git config http.proxy localhost:9090 # http:// is prefixed automatically
)
