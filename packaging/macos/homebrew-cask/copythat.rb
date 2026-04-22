# Homebrew cask for Copy That 2026.
#
# Lives in a dedicated `copythat-homebrew` tap (free to create and
# host on GitHub). Users install with:
#   brew tap copythat/copythat
#   brew install --cask copythat
#
# Homebrew auto-clears the Gatekeeper quarantine flag on cask
# installs, so the ad-hoc-signed `.app` bundle runs on first launch
# without the "unidentified developer" warning. `sha256` is filled
# in at publish time by the release helper.
cask "copythat" do
  version "0.1.0"
  sha256 arm:   "0000000000000000000000000000000000000000000000000000000000000000",
         intel: "0000000000000000000000000000000000000000000000000000000000000000"

  on_arm do
    url "https://github.com/MikesRuthless12/CopyThat2026/releases/download/v#{version}/CopyThat_#{version}_aarch64.dmg"
  end
  on_intel do
    url "https://github.com/MikesRuthless12/CopyThat2026/releases/download/v#{version}/CopyThat_#{version}_x64.dmg"
  end

  name "Copy That 2026"
  desc "Byte-exact cross-platform file copier (Rust + Tauri 2)"
  homepage "https://github.com/MikesRuthless12/CopyThat2026"

  livecheck do
    url :url
    strategy :github_latest
  end

  depends_on macos: ">= :monterey"

  app "Copy That 2026.app"

  zap trash: [
    "~/Library/Application Support/CopyThat2026",
    "~/Library/Preferences/com.copythat.desktop.plist",
    "~/Library/Caches/com.copythat.desktop",
  ]
end
