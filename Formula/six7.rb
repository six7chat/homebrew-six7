class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.0 - compatible with Six7 mobile app"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.24.tar.gz"
  sha256 "e6fe3a908e792cb73dc96e4d85ec612194b388f7cb4efd4070642655bf88a3a5"
  license "MIT"
  head "https://github.com/six7chat/homebrew-six7.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
