#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
host_build_dir="${SKYRIM_LLM_HOST_BUILD_DIR:-/host-vistularim/skyrim-llm-runtime-build}"

if ! command -v rsync >/dev/null 2>&1; then
  echo "rsync is required to export the Windows build tree." >&2
  exit 1
fi

mkdir -p "${host_build_dir}"

sync_tree() {
  local src="$1"
  local dst="$2"

  mkdir -p "${dst}"
  rsync -a --delete \
    --exclude 'build/' \
    --exclude 'CMakeUserPresets.json' \
    "${src}/" "${dst}/"
}

sync_tree "${repo_root}/mod" "${host_build_dir}/mod"
sync_tree "${repo_root}/skse-plugin" "${host_build_dir}/skse-plugin"

cat <<EOF
Exported Windows build tree to:
  ${host_build_dir}

Build from Windows in:
  E:\\Modding\\VistulaRim\\skyrim-llm-runtime-build\\skse-plugin
EOF
