class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.45"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.45"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.45/six7-0.7.45-arm64-apple-darwin.tar.gz"
      sha256 "c810bcfe32e6532b9c78fc848b4f22c3ba2a95b2b746d3a985ded1f57db895e5"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
