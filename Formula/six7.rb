class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.14"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.14"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.14/six7-0.7.14-arm64-apple-darwin.tar.gz"
      sha256 "1d14df32c844955a4cb44e9e61b286c52565a96c3fb518a267600e4ec9dd0e7d"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
