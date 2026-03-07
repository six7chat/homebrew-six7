class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.65"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.65"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.65/six7-0.7.65-arm64-apple-darwin.tar.gz"
      sha256 "236c402eab5dc31fe1f840c81e43b1d2a2086834cf10f88e885602eee3715052"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
