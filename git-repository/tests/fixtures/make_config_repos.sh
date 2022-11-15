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
  git config http.proxy localhost:9090
  git config http.proxyAuthMethod anyauth
  git config http.userAgent agentJustForHttp
)

git init http-config-empty-proxy
(cd http-config-empty-proxy
  git config http.proxy localhost:9090
  git config --add http.proxy "" # a value override disabling it later
)
