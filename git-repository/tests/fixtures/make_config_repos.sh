set -eu -o pipefail

git init http-config
(cd http-config
  git config http.extraHeader "ExtraHeader: value1"
  git config --add http.extraHeader ""
  git config --add http.extraHeader "ExtraHeader: value2"
  git config --add http.extraHeader "ExtraHeader: value3"
  git config http.followRedirects true
  git config http.lowSpeedLimit 5k
  git config http.lowSpeedTime 10
  git config http.postBuffer 8k
  git config http.proxy http://localhost:9090
  git config http.proxyAuthMethod basic
  git config http.userAgent agentJustForHttp
  git config gitoxide.http.connectTimeout 60k
)

git clone --shared http-config http-remote-override
(cd http-remote-override

  git config http.followRedirects initial

  git config http.proxy http://localhost:9090
  git config http.proxyAuthMethod basic

  git config remote.origin.proxy overridden
  git config remote.origin.proxyAuthMethod negotiate
)

git init http-proxy-empty
(cd http-proxy-empty
  git config http.followRedirects false

  git config http.proxy localhost:9090
  git config --add http.proxy "" # a value override disabling it later
)

git init http-proxy-auto-prefix
(cd http-proxy-auto-prefix
  git config http.proxy localhost:9090 # http:// is prefixed automatically
)

git init http-proxy-authenticated
(cd http-proxy-authenticated
  git config http.proxy user@localhost:9090
  cat <<EOF >> .git/config
[http]
  followRedirects
EOF
)
