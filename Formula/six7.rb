class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.55"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.55"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.55/six7-0.7.55-arm64-apple-darwin.tar.gz"
      sha256 "e4afc8428f59dde7a41ab679cf2d6daa84d3388e36c1a4b1b29550dd4ca82d4b"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
