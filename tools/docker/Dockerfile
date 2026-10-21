FROM xboxdev/nxdk:latest
RUN (apk add --no-cache bison flex make) 2>/dev/null || (apt-get update && apt-get install -y --no-install-recommends bison flex make && rm -rf /var/lib/apt/lists/*)
RUN bison --version | head -n1 && flex --version
