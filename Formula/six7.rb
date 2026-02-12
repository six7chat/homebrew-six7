class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.26"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.26"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.26/six7-0.7.26-arm64-apple-darwin.tar.gz"
      sha256 "4b512da90ac46d4b59560f9e989b0d5ce67c0a4d6f58d1c4b23d347548055252"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
