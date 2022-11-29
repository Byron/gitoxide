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

git init http-no-proxy
(cd http-no-proxy
  git config gitoxide.http.noProxy "no validation done here"
)


git init http-proxy-empty
(cd http-proxy-empty
  git config http.followRedirects false

  git config http.proxy localhost:9090
  git config --add http.proxy "" # a value override disabling it later
)

git init https-proxy-only
(cd https-proxy-only
  git config gitoxide.https.proxy https
)

git init gitoxide-http-proxy-only
(cd gitoxide-http-proxy-only
  git config gitoxide.http.proxy http-fallback
)

git init gitoxide-all-proxy-only
(cd gitoxide-all-proxy-only
  git config gitoxide.http.allProxy all-proxy-fallback
)

git init gitoxide-all-proxy
(cd gitoxide-all-proxy
  git config http.proxy http
  git config gitoxide.http.allProxy all-proxy-fallback
)

git init gitoxide-http-proxy
(cd gitoxide-http-proxy
  git config gitoxide.http.proxy http-fallback
  git config http.proxy http
)

git init https-proxy
(cd https-proxy
  git config gitoxide.https.proxy https
  git config --add http.proxy "http"  # only for HTTP urls
)

git init https-proxy-empty
(cd https-proxy-empty
  git config gitoxide.https.proxy https
  git config --add gitoxide.https.proxy "" # empty strings disable it
)

git init http-proxy-auto-prefix
(cd http-proxy-auto-prefix
  git config http.proxy localhost:9090 # http:// is prefixed automatically
)

git init http-verbose
(cd http-verbose
  git config gitoxide.http.verbose true
)

git init http-proxy-authenticated
(cd http-proxy-authenticated
  git config http.proxy user@localhost:9090
  cat <<EOF >> .git/config
[http]
  followRedirects
EOF
)

git init object-caches
(cd object-caches
  git config core.deltaBaseCacheLimit 128m
  git config gitoxide.objects.cacheLimit 16m
)

git init disabled-object-caches
(cd disabled-object-caches
  git config core.deltaBaseCacheLimit 0
  git config gitoxide.objects.cacheLimit 0
)
