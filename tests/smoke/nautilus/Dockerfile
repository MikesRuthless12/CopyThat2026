# Phase 7 follow-up — real Nautilus-runtime load test for the GNOME Files
# extension. ubuntu:20.04 pins the Nautilus 3.0 GIR typelib the extension
# targets (`gi.require_version("Nautilus", "3.0")`); modern distros ship 4.x.
#
# Built via tests/smoke/nautilus/run.sh, which stages this Dockerfile next
# to the extension + the check in a temp context (the extension lives under
# packaging/, outside this directory). CI skips the Dockerfile and runs the
# same steps natively in a `container: ubuntu:20.04` (see ci.yml).
FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        python3 \
        python3-gi \
        gir1.2-nautilus-3.0 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY copythat_nautilus.py /app/ext/copythat_nautilus.py
COPY nautilus_runtime_check.py /app/nautilus_runtime_check.py
ENV COPYTHAT_NAUTILUS_DIR=/app/ext
# Headless note: Gdk prints a benign "Unable to init server: Could not
# connect" to stderr (no X display), but the typelib import + MenuItem
# construction don't need one — the check still verifies and exits 0.
CMD ["python3", "/app/nautilus_runtime_check.py"]
